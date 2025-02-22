#BrainFetch Commands

BrainFetch implements the commands of Brainfuck, an esoteric turing-complete language written by Urban Muller, and adds its own.



## Command Descriptions

### `>`
- **Description**: Moves the memory pointer to the right by one cell.
- **Usage**: Use this command to access or modify the next memory cell.

### `<`
- **Description**: Moves the memory pointer to the left by one cell.
- **Usage**: Use this command to access or modify the previous memory cell.

### `+`
- **Description**: Increments the value at the current memory cell by one.
- **Usage**: Use this command to increase the value stored in the memory cell. If the cell value overflows (exceeds 255), it wraps around to 0.

### `-`
- **Description**: Decrements the value at the current memory cell by one.
- **Usage**: Use this command to decrease the value stored in the memory cell. If the cell value underflows (becomes less than 0), it wraps around to 255.

### `.`
- **Description**: Outputs the value at the current memory cell as a character.
- **Usage**: Use this command to print the character corresponding to the byte value in the current memory cell. For example, if the current cell contains `65`, it will output `A`.

### `,`
- **Description**: Accepts a single character input and stores it in the current memory cell.
- **Usage**: Use this command to read a character from standard input. The ASCII value of the input character is stored in the current memory cell.

### `@`
- **Description**: Fetches data from a URL stored in memory and stores the response in memory.
- **Usage**: Use this command to execute an HTTP GET request to the URL stored in the current memory cell. Ensure the URL is properly formatted. The response body will be stored starting from the next memory cell.

### `[`
- **Description**: Starts a loop. If the value at the current memory cell is zero, jumps to the corresponding `]` command.
- **Usage**: Use this command to initiate a loop. If the current cell value is zero, the interpreter will skip to the command after the matching `]`. This allows for conditional looping based on the value in the current memory cell.

### `]`
- **Description**: Ends a loop. If the value at the current memory cell is non-zero, jumps back to the corresponding `[` command.
- **Usage**: Use this command to terminate a loop. If the current cell value is not zero, the interpreter will jump back to the matching `[` command to repeat the loop.

### `|`
- **Description**: Moves the memory pointer left by the amount specified in the current memory cell.
- **Usage**: Use this command to move the memory pointer left by the value contained in the current memory cell. This allows for dynamic pointer movement based on cell values.

### `*`
- **Description**: Moves the memory pointer right by the amount specified in the current memory cell.
- **Usage**: Use this command to move the memory pointer right by the value contained in the current memory cell. This allows for dynamic pointer movement based on cell values.

### `#`
- **Description**: Resets the memory pointer to the starting position (cell 0).
- **Usage**: Use this command to quickly return the memory pointer to the first memory cell (cell 0).

