
pub enum Command {
	RemoveEntity(char, usize, usize),
	LoadNextMap(usize, usize),
	LoadMap(String),
	Exit(),
	ResetMap(),
}
