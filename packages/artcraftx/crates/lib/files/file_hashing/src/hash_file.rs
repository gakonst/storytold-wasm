use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

/// Streaming read size. 1 MiB keeps syscall counts low without holding a
/// large buffer per concurrent hash.
const READ_BUFFER_BYTES: usize = 1024 * 1024;

/// BLAKE3 of a file's contents, as 64 lowercase hex characters.
///
/// Streams the file; memory use is one buffer regardless of file size. This
/// is blocking I/O — call it from `spawn_blocking` in async contexts.
pub fn hash_file_blake3<P: AsRef<Path>>(path: P) -> std::io::Result<String> {
  let file = File::open(path)?;
  let mut reader = BufReader::with_capacity(READ_BUFFER_BYTES, file);
  let mut hasher = blake3::Hasher::new();
  let mut buffer = vec![0u8; READ_BUFFER_BYTES];
  loop {
    let read = reader.read(&mut buffer)?;
    if read == 0 {
      break;
    }
    hasher.update(&buffer[..read]);
  }
  Ok(hasher.finalize().to_hex().to_string())
}

/// BLAKE3 of in-memory bytes, as 64 lowercase hex characters.
pub fn hash_bytes_blake3(bytes: &[u8]) -> String {
  blake3::hash(bytes).to_hex().to_string()
}

#[cfg(test)]
mod tests {
  use super::*;

  // Known-answer test vector: blake3 of the empty input.
  const EMPTY_BLAKE3: &str = "af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262";

  #[test]
  fn known_answers() {
    assert_eq!(hash_bytes_blake3(b""), EMPTY_BLAKE3);
    assert_eq!(
      hash_bytes_blake3(b"hello world"),
      "d74981efa70a0c880b8d8c1985d075dbcbf679b99a5f9914e5aaf96b831a9e24",
    );
  }

  #[test]
  fn file_and_bytes_agree() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("data.bin");
    let contents = vec![7u8; 3 * 1024 * 1024 + 17]; // spans several read buffers
    std::fs::write(&path, &contents).unwrap();
    assert_eq!(hash_file_blake3(&path).unwrap(), hash_bytes_blake3(&contents));

    let empty = dir.path().join("empty.bin");
    std::fs::write(&empty, b"").unwrap();
    assert_eq!(hash_file_blake3(&empty).unwrap(), EMPTY_BLAKE3);
  }

  #[test]
  fn missing_file_errors() {
    assert!(hash_file_blake3("/definitely/not/a/real/file").is_err());
  }
}
