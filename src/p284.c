/*
 * @lc app=leetcode id=284 lang=c
 */

#include <stdbool.h>

struct Iterator
{
	// Returns true if the iteration has more elements.
	bool (*hasNext)();

	// Returns the next element in the iteration.
	int (*next)();
};

// @lc code=start
#include <stdlib.h>

struct PeekingIterator
{
	struct Iterator *iterator;
	bool hasPeeked;
	int nextValue;
};

struct PeekingIterator *Constructor(struct Iterator *iter)
{
	struct PeekingIterator *piter = malloc(sizeof(struct PeekingIterator));
	piter->iterator = iter;
	piter->hasPeeked = false;
	return piter;
}

int peek(struct PeekingIterator *obj)
{
	if (!obj->hasPeeked)
	{
		obj->hasPeeked = true;
		obj->nextValue = obj->iterator->next();
	}
	return obj->nextValue;
}

int next(struct PeekingIterator *obj)
{
	if (obj->hasPeeked) {
		obj->hasPeeked = false;
		return obj->nextValue;
	} else {
		return obj->iterator->next();
	}
}

bool hasNext(struct PeekingIterator *obj)
{
	return obj->hasPeeked || obj->iterator->hasNext();
}

/*
 * Your PeekingIterator struct will be instantiated and called as such:
 * PeekingIterator* obj = peekingIteratorCreate(arr, arrSize);
 * int param_1 = peek(obj);
 * int param_2 = next(obj);
 * bool param_3 = hasNext(obj);
 * peekingIteratorFree(obj);
 */

// @lc code=end
