#include "parser.h"
#include "data.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static int CountItemsInFile(FILE*);
static struct DigInstructionParseResult ParseDigInstructionFromLine(char*);

struct DigInstructions ParseFile(char* filename)
{
	struct DigInstructions noResult = {NULL, 0};

	struct DigInstruction* instructions = NULL;
	FILE* handler = fopen(filename, "r");
	int items = 0;

	if (handler) {
		// Count items in file and allocate memory
		items = CountItemsInFile(handler);
		if (items < 1) {
			return noResult;
		}
		instructions = (struct DigInstruction*) malloc(sizeof(struct DigInstruction) * items);

		// Rewind file handler
		rewind(handler);

		// Parse line by line
		int lineLength = 24;
		int lineIndex = 0;
		char* line = malloc(sizeof(char) * lineLength);
		while (1) {
			// Read line and trim the trailing newline
			fgets(line, lineLength, handler);
			if (feof(handler)) {
				break;
			}
			line[strcspn(line, "\n")] = '\0';

			// Parse and add to instructions if OK
			struct DigInstructionParseResult parse = ParseDigInstructionFromLine(line);
			if (!parse.ok) {
				continue;
			}
			instructions[lineIndex] = parse.di;
			lineIndex++;
		};

		free(line);
		line = NULL;
		fclose(handler);
		handler = NULL;
	}

	struct DigInstructions instructionsContainer = {instructions, items};
	return instructionsContainer;
}

static int CountItemsInFile(FILE* handler)
{
	int items = 0;
	char ch;
	while ((ch = fgetc(handler)) != EOF) {
		if (ch == '\n') {
			items++;
		}
	}

	return items;
}

static struct DigInstructionParseResult ParseDigInstructionFromLine(char* line)
{
#define DELIMITER " "
	int segment = 0;
	int ok = 1;

	enum Direction direction;
	unsigned int distance;
	char* color;

	char* split = strtok(line, DELIMITER);
	switch (*split) {
		case 'D': {
			direction = Down;
			break;
		}
		case 'U': {
			direction = Up;
			break;
		}
		case 'L': {
			direction = Left;
			break;
		}
		case 'R': {
			direction = Right;
			break;
		}
		default: {
			ok = 0;
		}
	}
	segment++;

	while (1) {
		if (ok == 0 || segment == 0 || segment > 2) {
			break;
		}

		split = strtok(NULL, DELIMITER);
		if (split == NULL || strlen(split) < 1) {
			break;
		}

		if (segment == 1) {
			distance = atoi(split);
			segment++;
		}

		else if (segment == 2) {
			color = malloc(sizeof(char) * (6 + 1));
			strncpy(color, &split[2], 6);
			segment++;
		}
	}

	struct DigInstruction di = {
		direction,
		distance,
		color,
	};
	struct DigInstructionParseResult result = {ok, di};
	return result;
}
