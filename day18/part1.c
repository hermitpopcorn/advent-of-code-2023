#include "data.h"
#include "parser.h"
#include "process.h"
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

	struct DigMap digMap = createDigMap(&parseResult);

	return 0;
}
