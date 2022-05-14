/*
 * @lc app=leetcode id=116 lang=c
 */

struct Node
{
	int val;
	struct Node *left;
	struct Node *right;
	struct Node *next;
};

// @lc code=start
#include <stdbool.h>
#include <stdlib.h>

struct Node *connect(struct Node *root)
{
	struct Node *head = root;
	while (true)
	{
		struct Node *sub_prev = NULL;
		struct Node *sub_first = NULL;
		for (struct Node *node = head; node != NULL; node = node->next)
		{
			if (node->left)
			{
				if (sub_prev)
					sub_prev->next = node->left;
				sub_prev = node->left;

				if (!sub_first)
					sub_first = node->left;
			}
			if (node->right)
			{
				if (sub_prev)
					sub_prev->next = node->right;
				sub_prev = node->right;

				if (!sub_first)
					sub_first = node->right;
			}
		}
		if (sub_first) {
			head = sub_first;
		} else {
			break;
		}
	}
	return root;
}

// @lc code=end