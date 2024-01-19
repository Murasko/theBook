# Ownership and the Stack/Heap
Both are parts of memory available to the code at runtime.

Pushing onto the stack is always faster than allocating memory to the heap. Thats because you don't have to search for a place, the location is always on top of the stack.

Accessing data in the heap is also slower than accessing data on the stack. There's a simple reason for it: Contemporary processors are faster if they jump less in memory. Since every time you need to access the data you need to follow the pointer, maybe have huge gaps inbetween the data, it's normally faster to access data that is close together, like on the stack.

When your code calls a function, the values passed into the function (potentially pointers to heap data, function variables, etc.) get pushed onto the stack. After the function is over, those values get popped off.

Keeping track of what parts are using what data on the heap, minimizing the amount of duplicate data and cleaning up unused data are all problems, addressed by the ownership concept. Once you understand ownership, you don't need to think anymore of the stack and heap often, but knowing that it's main purpose is to manage heap data can help explain why it works the way it does.


## The Stack
The Stack stores values in the order it gets them and removes them in the opposite order. Think of it like *last in, first out*.
A stack of plates is a scenario like this. When you add more plates to the stack, you take the last one put there first.
Adding or removing to the middle or bottom doesn't work well.

Adding data to the stack is called *pushing onto the stack*.
Removing data is called *popping of the stack*.

All data on the stack must have a known, fixed size. This means, all data that has a unknown size at compile time or a size that could get changed must be stored on the heap instead.

## The Heap
The heap is less organized. When you put data on the heap, you request a certain amount of space. THe memory allocator finds an empty spot with the right size, marks it as in use and returns a pointer (address of that memory location).
This process is called *allocating on the heap* and is sometimes abbreviated to just *allocating*. Pushing values onto the stack is not considered allocating.
Because the pointer is a fixed size, it can be stored on the stack. If you want to access the data, you need to follow the pointer.

Think of an restaurant. You state the number of people in your group and the waiter tries to find a table for you. If somebody arrives late, they can ask where you've been seated.

## Ownership Rules
- Each value in Rust has an owner
- There can only be one owner at a time
- When the owner goes out of scope, the value will be dropped

