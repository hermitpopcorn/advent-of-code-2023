enum Direction
{
	Up = 'U',
	Down = 'D',
	Right = 'R',
	Left = 'L',
};

struct DigInstruction
{
	enum Direction direction;
	unsigned int distance;
	char* color;
};

struct DigInstructionParseResult
{
	int ok;
	struct DigInstruction di;
};

struct DigInstructions
{
	struct DigInstruction* instructions;
	int size;
};
