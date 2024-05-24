#include "data.h"
#include <stdlib.h>
#include <stdio.h>

struct Area {
	unsigned int width;
	unsigned int length;
};

static struct Area getDigArea(struct DigInstructions* digInsts) {
	unsigned int width = 1, length = 1;
	int x = 0, y = 0;

	for (int i = 0; i < digInsts->size; i++) {
		struct DigInstruction* instr = &(digInsts->instructions[i]);

		switch (instr->direction) {
			case Up:
				y -= instr->distance;
				break;
			case Down:
				y += instr->distance;
				break;
			case Left:
				x -= instr->distance;
				break;
			case Right:
				x += instr->distance;
				break;
		}

		if (x + 1 > width)
			width = x + 1;
		if (y + 1 > length)
			length = y + 1;
	}

	struct Area area = {width, length};
	return area;
}

struct DigMap createDigMap(struct DigInstructions* digInsts) {
	// Blank map to return in case of failures
	struct DigMap dm = {0, 0, NULL};

	struct Area digArea = getDigArea(digInsts);
	printf("W: %d, L: %d\n", digArea.width, digArea.length);

	enum TerrainType* map = NULL;

	dm = (struct DigMap) {digArea.width, digArea.length, map};
	return dm;
}

unsigned int countTotalLavaValue(struct DigMap* dm) {
	return 0;
}
