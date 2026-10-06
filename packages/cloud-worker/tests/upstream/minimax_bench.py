# Pure functions extracted unchanged from the upstream benchmark; see ../../UPSTREAM.json.

TEXT_ENCODER = "qwen3vl_32b_minimax_h3_nvfp4_awq.safetensors"

VIDEO_VAE = "minimax_h3_video_vae_fp16.safetensors"

AUDIO_VAE = "minimax_h3_audio_vae_fp32.safetensors"

BENCH_IMAGE = "bench2_jp_01.jpg"

BENCH_REF_SET = ["bench2_jp_01.jpg", "bench2_jp_gate_01.png",
                 "bench2_ashitaka_01.jpg", "bench2_bebop_01.jpg",
                 "bench2_forest_01.jpg", "bench2_forest_02.jpg",
                 "bench2_volcano_01.jpg", "bench2_volcano_02.png",
                 "bench2_jp_02.jpg"]

def snap_length(seconds: float) -> int:
    """Duration in seconds -> valid H3 frame count (24 fps, n % 17 == 5, snapped up)."""
    n = max(5, round(seconds * 24))
    return n + (5 - (n % 17)) % 17

def build_graph(task, model_file, prompt, width, height, length, steps, seed,
                sampler="res_multistep", scheduler="simple", image=BENCH_IMAGE,
                ref_image_size="match", ref_count=1, ref_downscale=0.0,
                filename_prefix="bench/run"):
    g = {
        "1": {"class_type": "UNETLoader",
              "inputs": {"unet_name": model_file, "weight_dtype": "default"}},
        "2": {"class_type": "CLIPLoader",
              "inputs": {"clip_name": TEXT_ENCODER, "type": "minimax", "device": "default"}},
        "3": {"class_type": "VAELoader", "inputs": {"vae_name": VIDEO_VAE}},
        "4": {"class_type": "VAELoader", "inputs": {"vae_name": AUDIO_VAE}},
        "6": {"class_type": "KSamplerSelect", "inputs": {"sampler_name": sampler}},
        "7": {"class_type": "BasicScheduler",
              "inputs": {"model": ["1", 0], "scheduler": scheduler, "steps": steps,
                         "denoise": 1.0}},
        "8": {"class_type": "RandomNoise", "inputs": {"noise_seed": seed}},
        "9": {"class_type": "BasicGuider",
              "inputs": {"model": ["1", 0], "conditioning": ["5", 0]}},
        "11": {"class_type": "SamplerCustomAdvanced",
               "inputs": {"noise": ["8", 0], "guider": ["9", 0], "sampler": ["6", 0],
                          "sigmas": ["7", 0], "latent_image": ["5", 1]}},
        "12": {"class_type": "VAEDecode", "inputs": {"samples": ["11", 0], "vae": ["3", 0]}},
        "13": {"class_type": "VAEDecodeAudio", "inputs": {"samples": ["11", 0], "vae": ["4", 0]}},
        "14": {"class_type": "CreateVideo",
               "inputs": {"images": ["12", 0], "audio": ["13", 0], "fps": 24, "bit_depth": 8}},
        "15": {"class_type": "SaveVideo",
               "inputs": {"video": ["14", 0], "filename_prefix": filename_prefix,
                          "format": "auto", "codec": "auto"}},
    }
    common = {"prompt": prompt, "width": width, "height": height, "length": length}
    if task == "t2v":
        g["5"] = {"class_type": "MiniMaxH3ImageToVideo",
                  "inputs": {"clip": ["2", 0], "vae": ["3", 0], **common}}
    elif task == "i2v":
        g["10"] = {"class_type": "LoadImage", "inputs": {"image": image}}
        g["5"] = {"class_type": "MiniMaxH3ImageToVideo",
                  "inputs": {"clip": ["2", 0], "vae": ["3", 0], "first_frame": ["10", 0],
                             **common}}
    elif task == "ref2v":
        ref_inputs = {}

        def ref_source(nid_load, fname):
            g[nid_load] = {"class_type": "LoadImage", "inputs": {"image": fname}}
            if not ref_downscale:
                return [nid_load, 0]
            nid_scale = str(int(nid_load) + 40)
            g[nid_scale] = {"class_type": "ImageScaleToTotalPixels",
                            "inputs": {"image": [nid_load, 0],
                                       "upscale_method": "lanczos",
                                       "megapixels": ref_downscale,
                                       "resolution_steps": 1}}
            return [nid_scale, 0]

        if ref_count <= 1:
            ref_inputs["ref_images.ref_image_0"] = ref_source("10", image)
        else:
            for i in range(ref_count):
                ref_inputs[f"ref_images.ref_image_{i}"] = ref_source(
                    str(20 + i), BENCH_REF_SET[i % len(BENCH_REF_SET)])
        g["5"] = {"class_type": "MiniMaxH3ReferenceToVideo",
                  "inputs": {"clip": ["2", 0], "vae": ["3", 0], "audio_vae": ["4", 0],
                             "ref_image_size": ref_image_size,
                             **ref_inputs, **common}}
    else:
        raise ValueError(f"unknown task {task}")
    return g
