# Day 18: Snailfish

## Run

```
cat assets/input.txt | cargo run
```

## Bench

```
cat assets/input.txt | cargo bench
```

## Notes

### Part 1

The nice thing about today's input is that there is a 1:1 mapping between
each character, and it's meaning, so it won't require a tokenizer.

The problem here looks a lot like a binary tree. BUT although it looks
very simple at the first glance, trees have some inherent complexity.
E.g. how much memory is required for each tree element, if we  don't know how big
the children are?

After some contemplation, I decided to first attempt solving this with a linked list
data structure, where each element has a value, and a depth. The reason for choosing
a linked list is that I want to access adjacent elements to both 
directions. Also with a linked list, the variable memory requirement is isolated to 
just the linked list, and not each node. There are several downsides this too.
The most relevant in this is that the model is not going to feel as natural,
and some memory and ownership related complexity will be replaced by a more
complex domain model. Also, it will use a bit more memory, because commas will
be allocated the same amount of memory as other nodes, but it's just an integer, 
so this is not something I care about here. And changing the structure may become 
more complicated, but again, at least we won't have to worry about memory 
aspacts such as references/pointers/garbage collection (if we had one).

I also started writing an unit test for this, where I test that I can get the 
original value back from the linked list. If I am unable to do that, then I 
am losing some data, and the changes are it will not be possible to solve 
this challenge. Writing this unit test revealed that I should not forget 
about the commas, so I changed the depth and value pair to be an enumerator with
two possible values, a number with a depth and value, or a comma with a depth.

Then I read over the linked list documentation, and the instructions, and came to
realize that I am not really gaining anything with a linked list, since
modifying it in the middle is an expensive operation. So I will just switch to 
using Vecs. And since there will be a lot of traversal from left to right, it 
might make sense to reverse the order, and use it as a stack instead.

After implementing it, I can definitely say I was right that it made the the 
domain model hard to reason about, and the resulting codebase is pretty messy.
But I am quite happy that I was able to solve today's puzzle with this approach.

### Part 2

More of same thing, just some minor modifications.
Since the values are non-symmetrical, I just brute forced the answers,
but I suspect there is a faster way.
