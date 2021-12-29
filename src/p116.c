/*
 * @lc app=leetcode id=116 lang=rust
 */

struct Node
{
    int val;
    struct Node *left;
    struct Node *right;
    struct Node *next;
};

// @lc code=start
#include <stdlib.h>

static size_t calc_depth(struct Node *root)
{
    if (root == NULL)
    {
        return 0;
    }
    else
    {
        return calc_depth(root->left) + 1;
    }
}

static void fill_buf(struct Node **buf, size_t buf_size, struct Node *root)
{
    buf[0] = root;
    size_t beg = 0;
    size_t end = 1;
    size_t new_end = end;
    while (beg < buf_size)
    {
        for (size_t i = beg; i < end; i++)
        {
            if (i + 1 < end)
            {
                buf[i]->next = buf[i + 1];
            }
            else
            {
                buf[i]->next = NULL;
            }
            if (new_end < buf_size)
            {
                buf[new_end++] = buf[i]->left;
                buf[new_end++] = buf[i]->right;
            }
        }
        if (end == new_end)
        {
            break;
        }
        beg = end;
        end = new_end;
    }
}

struct Node *connect(struct Node *root)
{
    size_t depth = calc_depth(root);
    size_t buf_size = 1;
    for (size_t i = 0; i < depth; i++)
    {
        buf_size *= 2;
    }
    --buf_size;
    struct Node **buf = malloc(buf_size * sizeof(struct Node *));
    fill_buf(buf, buf_size, root);
    free(buf);
    return root;
}

// @lc code=end
