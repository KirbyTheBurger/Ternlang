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
