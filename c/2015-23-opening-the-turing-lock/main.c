// https://adventofcode.com/2015/day/23

#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>

typedef enum {
        INST_HALF,
        INST_TRIPLE,
        INST_INCREMENT,
        INST_JUMP,
        INST_JUMP_IF_EVEN,
        INST_JUMP_IF_ONE,
} instruction_type;

typedef enum {
        REG_A,
        REG_B,
} register_name;

typedef struct {
        register_name reg;
        int32_t offset;
} jump_target;

typedef struct {
        instruction_type type;
        union {
                register_name half_reg;
                register_name triple_reg;
                register_name increment_reg;
                int32_t jump_offset;
                jump_target jump_if_even_target;
                jump_target jump_if_one_target;
        };
} instruction;

void print_register(register_name reg) {
        switch (reg) {
        case REG_A:
                printf("REG_A");
                break;
        case REG_B:
                printf("REG_B");
                break;
        }
}

void print_instruction(instruction *instruction)
{
        printf("Instruction { INSTRUCTION_TYPE: ");
        switch (instruction->type) {
        case INST_HALF:
                printf("INST_HALF, half_reg: ");
                print_register(instruction->half_reg);
                break;
        case INST_TRIPLE:
                printf("INST_TRIPLE, triple_reg: ");
                print_register(instruction->triple_reg);
                break;
        case INST_INCREMENT:
                printf("INST_INCREMENT, increment_reg: ");
                print_register(instruction->increment_reg);
                break;
        case INST_JUMP:
                printf("INST_JUMP, offset: %d", instruction->jump_offset);
                break;
        case INST_JUMP_IF_EVEN:
                printf("INST_JUMP_IF_EVEN, jump_if_even_target: { reg: ");
                print_register(instruction->jump_if_even_target.reg);
                printf(", offset: %d", instruction->jump_if_even_target.offset);
                break;
        case INST_JUMP_IF_ONE:
                printf("INST_JUMP_IF_ONE, jump_if_one_target: { reg: ");
                print_register(instruction->jump_if_one_target.reg);
                printf(", offset: %d", instruction->jump_if_one_target.offset);
                break;
        }
        printf(" }\n");
}

register_name parse_register(char **input)
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

int32_t parse_offset(char **input)
{
        int32_t ret;
        // N.B. %n docs: No input is consumed. The corresponding
        // argument shall be a pointer to signed integer into which is
        // to be written the number of characters read from the input
        // stream so far by this call to the fscanf function
        int bytes_read;
        sscanf(*input, "%d%n", &ret, &bytes_read);
        *input += bytes_read;

        return ret;
}

instruction parse_instruction(char **input)
{
        // Match based on instruction name
        instruction instruction;
        if (strncmp(*input, "hlf", 3) == 0) {
                instruction.type = INST_HALF;
                *input += 4;
                instruction.half_reg = parse_register(input);
        } else if (strncmp(*input, "tpl", 3) == 0) {
                instruction.type = INST_TRIPLE;
                *input += 4;
                instruction.triple_reg = parse_register(input);
        } else if (strncmp(*input, "inc", 3) == 0) {
                instruction.type = INST_INCREMENT;
                *input += 4;
                instruction.increment_reg = parse_register(input);
        } else if (strncmp(*input, "jmp", 3) == 0) {
                instruction.type = INST_JUMP;
                *input += 4;
                instruction.jump_offset = parse_offset(input);
        } else if (strncmp(*input, "jie", 3) == 0) {
                instruction.type = INST_JUMP_IF_EVEN;
                *input += 4;
                instruction.jump_if_even_target.reg = parse_register(input);

                // Assert comma and space
                assert(*input[0] == ',');
                *input += 1;
                assert(*input[0] == ' ');
                *input += 1;

                instruction.jump_if_even_target.offset = parse_offset(input);
        } else if (strncmp(*input, "jio", 3) == 0) {
                instruction.type = INST_JUMP_IF_ONE;
                *input += 4;
                instruction.jump_if_one_target.reg = parse_register(input);

                // Assert comma and space
                assert(*input[0] == ',');
                *input += 1;
                assert(*input[0] == ' ');
                *input += 1;

                instruction.jump_if_one_target.offset = parse_offset(input);
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
        instruction *instructions;
} instructions_array;

instructions_array instructions_array_create()
{
        size_t capacity = 2;
        instruction *instructions = malloc(capacity * sizeof(*instructions));
        instructions_array array = {
                .len = 0,
                .capacity = capacity,
                .instructions = instructions,
        };
        return array;
}

void instructions_array_append(instructions_array *instructions, instruction instruction)
{
        if (instructions->capacity == instructions->len) {
                instructions->capacity *= 2;
                size_t new_size = instructions->capacity * sizeof(instructions_array);
                instructions->instructions = realloc(instructions->instructions, new_size);
        }

        instructions->instructions[instructions->len] = instruction;
        instructions->len += 1;
}

instructions_array parse_instructions(char *input)
{
        instructions_array instructions = instructions_array_create();

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

uint32_t *select_register(uint32_t *reg_a, uint32_t *reg_b, register_name reg)
{
        switch (reg) {
        case REG_A:
                return reg_a;
        case REG_B:
                return reg_b;
        default:
                fprintf(stderr, "Unknown register: %d\n", reg);
                exit(1);
        }
}

uint32_t simulation(instructions_array instructions, uint32_t a_start)
{
        uint32_t reg_a = a_start;
        uint32_t reg_b = 0;
        size_t pc = 0;

        uint32_t *current_reg;

        while (pc < instructions.len) {
                instruction instruction = instructions.instructions[pc];

                // printf("a = %u, b = %u, pc = %lu, ", reg_a, reg_b, pc);
                // print_instruction(&instruction);

                switch (instruction.type) {
                case INST_HALF:
                        current_reg = select_register(&reg_a, &reg_b, instruction.half_reg);
                        *current_reg /= 2;
                        pc++;
                        break;
                case INST_TRIPLE:
                        current_reg = select_register(&reg_a, &reg_b, instruction.triple_reg);
                        *current_reg *= 3;
                        pc++;
                        break;
                case INST_INCREMENT:
                        current_reg = select_register(&reg_a, &reg_b, instruction.increment_reg);
                        *current_reg += 1;
                        pc++;
                        break;
                case INST_JUMP:
                        pc += instruction.jump_offset;
                        break;
                case INST_JUMP_IF_EVEN:
                        current_reg = select_register(&reg_a, &reg_b, instruction.jump_if_even_target.reg);
                        if (*current_reg % 2 == 0) {
                                pc += instruction.jump_if_even_target.offset;
                        } else {
                                pc++;
                        }
                        break;
                case INST_JUMP_IF_ONE:
                        current_reg = select_register(&reg_a, &reg_b, instruction.jump_if_one_target.reg);
                        if (*current_reg == 1) {
                                pc += instruction.jump_if_one_target.offset;
                        } else {
                                pc++;
                        }
                        break;
                }
        }

        return reg_b;
}

const char *TEST_INPUT = "inc a\n\
jio a, +2\n\
tpl a\n\
inc a";

const char *REAL_INPUT = "jio a, +19\n\
inc a\n\
tpl a\n\
inc a\n\
tpl a\n\
inc a\n\
tpl a\n\
tpl a\n\
inc a\n\
inc a\n\
tpl a\n\
tpl a\n\
inc a\n\
inc a\n\
tpl a\n\
inc a\n\
inc a\n\
tpl a\n\
jmp +23\n\
tpl a\n\
tpl a\n\
inc a\n\
inc a\n\
tpl a\n\
inc a\n\
inc a\n\
tpl a\n\
inc a\n\
tpl a\n\
inc a\n\
tpl a\n\
inc a\n\
tpl a\n\
inc a\n\
inc a\n\
tpl a\n\
inc a\n\
inc a\n\
tpl a\n\
tpl a\n\
inc a\n\
jio a, +8\n\
inc b\n\
jie a, +4\n\
tpl a\n\
inc a\n\
jmp +2\n\
hlf a\n\
jmp -7";

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
        printf("sizeof(instruction_type) = %zu\n", sizeof(instruction_type));
        printf("sizeof(register) = %zu\n", sizeof(register_name));
        printf("sizeof(jump_target) = %zu\n", sizeof(jump_target));
        printf("sizeof(instruction) = %zu\n", sizeof(instruction));
        printf("sizeof(instructions_array) = %zu\n", sizeof(instructions_array));

        char *hlf_a_input = "hlf a";
        instruction half_a = parse_instruction(&hlf_a_input);
        print_instruction(&half_a);

        char *jie_input = "jie a, -123";
        instruction jump_if_even = parse_instruction(&jie_input);
        print_instruction(&jump_if_even);

        // Actual answer
        instructions_array instructions = parse_instructions((char *) REAL_INPUT);

        uint32_t part1 = simulation(instructions, 0);
        printf("part1: %u\n", part1);

        uint32_t part2 = simulation(instructions, 1);
        printf("part2: %u\n", part2);
}
