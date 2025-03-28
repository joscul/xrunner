
pub enum Command {
	RemoveEntity(char, usize, usize),
	LoadMap(String, char),
	Exit(),
	ResetMap(),
}
