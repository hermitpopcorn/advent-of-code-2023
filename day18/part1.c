#include "data.h"
#include "parser.h"
#include <stdio.h>

int main(int argc, char* argv[])
{
	char* filename = "input/example.txt";
	if (argc >= 2) {
		filename = argv[1];
	}
	struct DigInstructions parseResult = ParseFile(filename);

	if (parseResult.size < 1) {
		printf("Could not find dig instructions.\n");
	}

	for (int i = 0; i < parseResult.size; i++) {
		struct DigInstruction* instruction = &parseResult.instructions[i];
		printf("Dig Instruction %d: %c | %d | #%s\n", i + 1, instruction->direction, instruction->distance, instruction->color);
	}

	return 0;
}
