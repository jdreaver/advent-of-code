// https://adventofcode.com/2015/day/23

#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef enum {
        INST_HALF,
        INST_TRIPLE,
        INST_INCREMENT,
        INST_JUMP,
        INST_JUMP_IF_EVEN,
        INST_JUMP_IF_ODD,
} INSTRUCTION_TYPE;

typedef enum {
        REG_A,
        REG_B,
} REGISTER;

typedef struct {
        REGISTER reg;
        int offset;
} JumpTarget;

typedef struct {
        INSTRUCTION_TYPE type;
        union {
                REGISTER half_reg;
                REGISTER triple_reg;
                REGISTER increment_reg;
                int jump_offset;
                JumpTarget jump_if_even_target;
                JumpTarget jump_if_odd_target;
        } data;
} Instruction;

void print_register(REGISTER reg) {
        switch (reg) {
        case REG_A:
                printf("REG_A");
                break;
        case REG_B:
                printf("REG_B");
                break;
        }
}

void print_instruction(Instruction *instruction)
{
        printf("Instruction { INSTRUCTION_TYPE: ");
        switch (instruction->type) {
        case INST_HALF:
                printf("INST_HALF, data.half_reg: ");
                print_register(instruction->data.half_reg);
                break;
        case INST_TRIPLE:
                printf("INST_TRIPLE, data.triple_reg: ");
                print_register(instruction->data.triple_reg);
                break;
        case INST_INCREMENT:
                printf("INST_INCREMENT, data.increment_reg: ");
                print_register(instruction->data.increment_reg);
                break;
        case INST_JUMP:
                printf("INST_JUMP, offset: %d", instruction->data.jump_offset);
                break;
        case INST_JUMP_IF_EVEN:
                printf("INST_JUMP_IF_EVEN, data.jump_if_even_target: { reg: ");
                print_register(instruction->data.jump_if_even_target.reg);
                printf(", offset: %d", instruction->data.jump_if_even_target.offset);
                break;
        case INST_JUMP_IF_ODD:
                printf("INST_JUMP_IF_ODD, data.jump_if_odd_target: { reg: ");
                print_register(instruction->data.jump_if_odd_target.reg);
                printf(", offset: %d", instruction->data.jump_if_odd_target.offset);
                break;
        }
        printf(" }\n");
}

REGISTER parse_register(char **input)
{
        switch (*input[0]) {
        case 'a':
                *input += 1;
                return REG_A;
        case 'b':
                *input += 1;
                return REG_B;
        default:
                fprintf(stderr, "Unknown register: %c\b", *input[0]);
                exit(1);
        }
}

int parse_offset(char **input)
{
        int ret;
        // N.B. %n docs: No input is consumed. The corresponding
        // argument shall be a pointer to signed integer into which is
        // to be written the number of characters read from the input
        // stream so far by this call to the fscanf function
        int bytes_read;
        sscanf(*input, "%d%n", &ret, &bytes_read);
        *input += bytes_read;

        return ret;
}

Instruction parse_instruction(char **input)
{
        // Match based on instruction name
        Instruction instruction;
        if (strncmp(*input, "hlf", 3) == 0) {
                instruction.type = INST_HALF;
                *input += 4;
                instruction.data.half_reg = parse_register(input);
        } else if (strncmp(*input, "tpl", 3) == 0) {
                instruction.type = INST_TRIPLE;
                *input += 4;
                instruction.data.triple_reg = parse_register(input);
        } else if (strncmp(*input, "inc", 3) == 0) {
                instruction.type = INST_INCREMENT;
                *input += 4;
                instruction.data.increment_reg = parse_register(input);
        } else if (strncmp(*input, "jmp", 3) == 0) {
                instruction.type = INST_JUMP;
                *input += 4;
                instruction.data.jump_offset = parse_offset(input);
        } else if (strncmp(*input, "jie", 3) == 0) {
                instruction.type = INST_JUMP_IF_EVEN;
                *input += 4;
                instruction.data.jump_if_even_target.reg = parse_register(input);

                // Assert comma and space
                assert(*input[0] == ',');
                *input += 1;
                assert(*input[0] == ' ');
                *input += 1;

                instruction.data.jump_if_even_target.offset = parse_offset(input);
        } else if (strncmp(*input, "jio", 3) == 0) {
                instruction.type = INST_JUMP_IF_ODD;
                *input += 4;
                instruction.data.jump_if_odd_target.reg = parse_register(input);

                // Assert comma and space
                assert(*input[0] == ',');
                *input += 1;
                assert(*input[0] == ' ');
                *input += 1;

                instruction.data.jump_if_odd_target.offset = parse_offset(input);
        } else {
                fprintf(stderr, "Unknown instruction: %s\b", *input);
                exit(1);
        }

        return instruction;
}

// Auto expanding array of instructions.
typedef struct {
        size_t len;
        size_t capacity;
        Instruction *instructions;
} InstructionsArray;

InstructionsArray instructions_array_create()
{
        size_t capacity = 2;
        Instruction *instructions = malloc(capacity * sizeof(*instructions));
        InstructionsArray array = {
                .len = 0,
                .capacity = capacity,
                .instructions = instructions,
        };
        return array;
}

void instructions_array_append(InstructionsArray *instructions, Instruction instruction)
{
        if (instructions->capacity == instructions->len) {
                instructions->capacity *= 2;
                size_t new_size = instructions->capacity * sizeof(InstructionsArray);
                instructions->instructions = realloc(instructions->instructions, new_size);
        }

        instructions->instructions[instructions->len] = instruction;
        instructions->len += 1;
}

InstructionsArray parse_instructions(char *input)
{
        InstructionsArray instructions = instructions_array_create();

        char **input_ptr = &input;
        while (1) {
                instructions_array_append(&instructions, parse_instruction(input_ptr));

                if (**input_ptr == '\0') {
                        break;
                } else if (**input_ptr == '\n') {
                        *input_ptr += 1;
                } else {
                        fprintf(stderr, "Unexpected rest of input: %s\n", *input_ptr);
                        exit(1);
                }
        }

        return instructions;
}

const char *TEST_INPUT = "inc a\n\
jio a, +2\n\
tpl a\n\
inc a";

const char *REAL_INPUT = "";

int run_tests()
{
        printf("Tests successful!\n");
        return 0;
}

int main(int argc, char* argv[])
{
        if (argc == 2) {
                if (strcmp(argv[1], "test") == 0) {
                        return run_tests();
                } else {
                        fprintf(stderr, "Usage: main [test]\n");
                        return 1;
                }
        }

        // Test commands
        printf("sizeof(INSTRUCTION_TYPE) = %zu\n", sizeof(INSTRUCTION_TYPE));
        printf("sizeof(REGISTER) = %zu\n", sizeof(REGISTER));
        printf("sizeof(JumpTarget) = %zu\n", sizeof(JumpTarget));
        printf("sizeof(Instruction) = %zu\n", sizeof(Instruction));

        char *hlf_a_input = "hlf a";
        Instruction half_a = parse_instruction(&hlf_a_input);

        char *jie_input = "jie a, -123";
        Instruction jump_if_even = parse_instruction(&jie_input);
        print_instruction(&jump_if_even);

        InstructionsArray instructions = parse_instructions(TEST_INPUT);

        printf("instructions:\n");
        for (size_t i = 0; i < instructions.len; i++) {
                print_instruction(&instructions.instructions[i]);
        }
}
