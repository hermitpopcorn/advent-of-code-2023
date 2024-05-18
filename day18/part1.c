#include "data.h"
#include "parser.h"
#include <stdio.h>

int main()
{
	struct DigInstructions parseResult = ParseFile("input/example.txt");

	for (int i = 0; i < parseResult.size; i++) {
		struct DigInstruction* instruction = &parseResult.instructions[i];
		printf("Dig Instruction %d: %c | %d | #%s\n", i + 1, instruction->direction, instruction->distance, instruction->color);
	}

	return 0;
}
