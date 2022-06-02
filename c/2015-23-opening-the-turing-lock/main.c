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
        assert(strlen(*input) > 0);
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

Instruction parse_instruction(char *input)
{
        char *rest = input;

        // Match based on instruction name
        Instruction instruction;
        if (strncmp(rest, "hlf", 3) == 0) {
                instruction.type = INST_HALF;
                rest += 4;
                instruction.data.half_reg = parse_register(&rest);
        } else if (strncmp(rest, "tpl", 3) == 0) {
                instruction.type = INST_TRIPLE;
                rest += 4;
                instruction.data.triple_reg = parse_register(&rest);
        } else if (strncmp(rest, "inc", 3) == 0) {
                instruction.type = INST_INCREMENT;
                rest += 4;
                instruction.data.increment_reg = parse_register(&rest);
        } else if (strncmp(rest, "jmp", 3) == 0) {
                instruction.type = INST_JUMP;
                rest += 4;
                instruction.data.jump_offset = parse_offset(&rest);
        } else if (strncmp(rest, "jie", 3) == 0) {
                instruction.type = INST_JUMP_IF_EVEN;
                rest += 4;
                instruction.data.jump_if_even_target.reg = parse_register(&rest);

                // Assert comma and space
                assert(rest[0] == ',');
                rest += 1;
                assert(rest[0] == ' ');
                rest += 1;

                instruction.data.jump_if_even_target.offset = parse_offset(&rest);
        } else if (strncmp(rest, "jio", 3) == 0) {
                instruction.type = INST_JUMP_IF_ODD;
                rest += 4;
                instruction.data.jump_if_odd_target.reg = parse_register(&rest);

                // Assert comma and space
                assert(rest[0] == ',');
                rest += 1;
                assert(rest[0] == ' ');
                rest += 1;

                instruction.data.jump_if_odd_target.offset = parse_offset(&rest);
        } else {
                fprintf(stderr, "Unknown instruction: %s\b", rest);
                exit(1);
        }

        // Assert no more input
        assert(rest[0] == '\0');

        return instruction;
}


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

        Instruction half_a = parse_instruction("hlf a");
        print_instruction(&half_a);
        Instruction triple_b = parse_instruction("tpl_b");
        print_instruction(&triple_b);

        Instruction jump = parse_instruction("jmp -345");
        print_instruction(&jump);
        jump = parse_instruction("jmp 1234");
        print_instruction(&jump);

        //Instruction jump_if_even = { .type = INST_JUMP_IF_EVEN };
        Instruction jump_if_even = parse_instruction("jie a, -123");
        print_instruction(&jump_if_even);

        Instruction jump_if_odd = parse_instruction("jio a, -123");
        print_instruction(&jump_if_odd);
}
