# asm-matcher

Analyzes similarity between assembly functions

## Background

When game modders need to identify specific functions within a game's executable at runtime, they often resort to one of
two options:

- Search for a specific hexadecimal string, then reuse the offset of its location as long as the game doesn't update
    - Pros: Easy and reliable
    - Cons: Usually only works for a specific game, not across an entire game engine
- Search for a masked hexadecimal pattern, which may be reused across updates
    - Pros: May work across multiple games
    - Cons: Unreliable, will require updating and alternative patterns for games with modified game engines

While programming [UnrealVR](https://github.com/TheNewJavaman/unreal-vr), a mod that adds virtual reality support to
flatscreen Unreal Engine games, I realized that in order to create a "universal" mod, a new assembly scanning system
would need to be created; thus, PatternStreams (PS) was created. PS inputs a set of rules, then searches across memory
space for matches. These rules usually involve masked hexadecimal patterns, but add features such as chaining patterns
and filter functions. PS worked well enough as a temporary assembly scanner solution, but ideally, assembly matching
shouldn't be such an
intensive manual process.

asm-matcher is my "ideal" solution to this problem, but there are numerous improvements that can still be made. At a
high level, it breaks down assembly functions into logical-flow chunks, similar to existing disassemblers' graphs, then
breaks each chunk into its opcodes, substituting literal registers or constants for placeholders.

## Signature Generation

1. Break binary into opcodes
    1. This project relies on [opcodesDB](https://github.com/MahdiSafsafi/opcodesDB), which generates JSON
       files detailing each x86 instruction. asm-matcher uses this output for its own opcode analysis
    2. To use opcodesDB, clone asm-matcher with submodules, then: `cd opcodesDB && perl x86.pl`
2. Use jumps within the function to break it apart into logical chunks (flow)
3. Substitute literal registers or constants for placeholders

## Workflow

*(The steps listed below are written for Unreal Engine game modding, but can be generalized to other applications such
as malware analysis)*

1. Create function signature
    1. Create a template game in Unreal Engine
    2. Package the game for Shipping with debug symbols enabled
    3. Use a disassembler such as [rizin](https://rizin.re/) to search for a function
    4. Export the function's raw hexadecimal to a separate binary file (ex. `... > aactor_tick.bin`)
    5. Run this tool to generate a signature (ex. `asm-matcher aactor_tick.bin aactor_tick.matcher`)
        1. Doing this step ahead of time is recommended to verify that all opcodes in the function are supported by this
           library. If asm-matcher doesn't recognize an opcode, it'll treat it as an unmasked pattern, which can lead to
           false negatives.
2. Apply signature to memory at runtime (API TBD)
    1. Load the matcher file
    2. Determine the bounds of memory you wish to search
        1. On Windows, you can use the base module pointer and module size to figure this out
    3. Run the scanner function, which will return pointers with matches over a certain threshold of similarity (TBD)