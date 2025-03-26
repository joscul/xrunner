
pub enum Command {
	RemoveEntity(char, usize, usize),
	LoadMap(String),
	Exit(),
	ResetMap(),
}
