use crate::*;
pub(crate) fn sha(bytes: &[u8]) -> String { format!("{:x}", Sha256::digest(bytes)) }
pub(crate) fn read_pinned(path: &Path, expected: &str) -> Result<Vec<u8>> {
    if expected.len()!=64 || !expected.bytes().all(|c| c.is_ascii_digit() || (b'a'..=b'f').contains(&c)) {return Err("invalid supervisor SHA256".into());}
    let file=fs::File::open(path)?; let mut bytes=Vec::new(); file.take(MAX_FILE+1).read_to_end(&mut bytes)?;
    if bytes.len() as u64>MAX_FILE {return Err("artifact exceeds source read ceiling".into());}
    if sha(&bytes)!=expected {return Err(format!("host artifact identity mismatch: {}",path.display()).into());} Ok(bytes)
}
pub(crate) fn artifact(a: &Artifact) -> Result<Vec<u8>> {read_pinned(&a.path,&a.sha256)}
pub(crate) fn decode<T: Deserializable+Tagged>(bytes: &[u8]) -> Result<T> {
    let mut cursor=Cursor::new(bytes);let value=tagged_deserialize(&mut cursor)?;
    if cursor.position()!=bytes.len() as u64 {return Err("outer tagged decoder trailing bytes".into());}Ok(value)
}
pub(crate) fn encoded<T: Serializable+Tagged>(value:&T)->Result<Vec<u8>> {let mut bytes=Vec::new();tagged_serialize(value,&mut bytes)?;Ok(bytes)}
pub(crate) fn exclusive(path:&Path,bytes:&[u8])->Result<()> {let mut f=OpenOptions::new().write(true).create_new(true).open(path)?;f.write_all(bytes)?;f.sync_all()?;Ok(())}
