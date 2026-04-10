## Introduction  
Ternlang is a stack based programming language, which means that you're able to push and pop variables to and from the stack, and that all instructions are operated on the stack. All values are in ternary, which is a base-3 number system.  
  
## Pushing to the stack  
To push a value to the stack, use the `>` instruction, followed by either a character (can be any valid unicode character), or a number in ternary. Example:  
```ternlang
>201
```  
*Push `0t201` to the stack (`19` in decimal)*  
  
```ternlang
>a
```  
*Push the character `a` to the stack*  
  
## Popping from the stack  
If you want to manually pop a value from the stack, use the `<` instruction. Popping when the stack is empty will give a runtime error.  
```ternlang
>122<
```  
*First push `0t122` to the stack, then pop from the stack. The stack is now empty*  
  
## Printing  
To print a value, use the `!` instruction. This pops a value from the stack and prints it to the temrinal (as an unicode character).  
```ternlang
>10121!
```  
*output:* `a`  
  
```ternlang
>d>l>r>o>w>o>l>l>e>h!!!!!!!!!!
```  
*output:* `helloworld`  
  
## Getting input  
To get the user's input, use the `?` instruction:  
```ternlang
?!
```  
*Read the user's input and then print it*  
  
```ternlang
>e?!!
```  
*Read the user's input, then print it followed by an `e`*  
  
## Arithmetic  
Each arithmetic instruction pops 2 values from the stack and executes that operation on them, then pushes the new value to the stack.  
### Addition  
Add 2 numbers using `+`:  
```ternlang
>1>2+
```  
*Value stored on the stack is now `3`*  
  
### Subtraction  
Subtract a number from another using `-`:  
```ternlang
>6>2-
```  
*Value stored on the stack is now `4`*  
  
### Multiplication  
Multiply 2 numbers using `*`:  
```ternlang
>3>4*
```  
*Value stored on the stack is now `12`  
  
### Division  
Divide a number by another using `/`:  
```ternlang
>8>4/
```  
*Value stored on the stack is now `2`  
