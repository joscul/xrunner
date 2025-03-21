
pub enum Command {
	RemoveEntity(char, usize, usize),
	LoadNextMap(usize, usize),
	Exit(),
	ResetMap(),
}
