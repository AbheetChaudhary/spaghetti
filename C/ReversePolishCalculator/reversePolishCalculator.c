#include<stdio.h>
#include<ctype.h>
#include<stdlib.h>
#include<math.h>
#include<string.h>

#define NUMBER '\0' // a signal that a number id encountered in the input stream
#define VARIABLE 'v' // a signal that a variable id encountered in the input stream
#define MAXOP 100 // maximum number of operands an operator can have, 2 for + -
                  // * /

#define TOP "top" // ID = 1
#define SWAP "swap" // ID = 2
#define EMPTY "empty" // ID = 3
#define LET "let" // ID = 4
#define SIN "sin" // ID = 101
#define COS "cos" // ID = 102
#define TAN "tan" // ID = 103
#define EXP "exp" // ID = 111
#define POW "pow" // ID = 112

double pop(void);
void push(double);
char getop(char[]);
int getcommandID(void);
void print(double);

void _top(void);
void _swap(void);
void _empty(void);
void _let(void);

double lastPrint;

#define STACKDEPTH 100 // maximum number of operators an operand can take

int sp = 0; // stack position of the first empty slot
double Stack[STACKDEPTH];

// for global/static scope arrays initialize to 0
double variables[26]; // variables will be held here
int variableExists[26]; // 1 if variable exists, 0 if not

int main() {
  double op2; // will be used to get the second operator in * and /
  double op1; // will be used when operator is %

  // the type of non-space not-tab character cluster i.e. operator operand or
  // newline
  int type;   

  // operands will be stored as string before converting them in double and
  // pushing on stack
  char s[MAXOP];

  int commandID;
  char command[100];

  while((type = getop(s)) != EOF) {
    switch(type) {
      case NUMBER:
        // if it is a number then convert it to double and push on stack
        push(atof(s));
        break;

      case VARIABLE:
        if(variableExists[tolower(s[0]) - 'a'] == 1) {
          push(variables[tolower(s[0] - 'a')]);
        } else {
          printf("error: variable %c does not exist, fallback 0 is used\n", s[0]);
          push(variables[tolower(s[0] - 'a')]);
        }
        break;

      case '+':
        // order of popping the stack does not matter here
        push(pop() + pop());
        break;

      case '-':
        // order of popping the stack does matter here thats why op2 is used
        op2 = pop();
        push(pop() - op2);
        break;

      case '*':
        // order of popping the stack does not matter here
        push(pop() * pop());
        break;

      case '/':
        // order of popping the stack does matter here thats why op2 is used
        op2 = pop();
        if(op2 == 0.0) {
          printf("ERROR: divide by zero\n");
        } else {
          push(pop() / op2);
        }
        break;

      case '%':
        op2 = pop();
        op1 = pop();
        if(op2 != floor(op2) || op1 != floor(op1)) {
          printf("alert: non-integer operator used with %%\n, truncation happened");
        }
        if(op2 == 0 || floor(op2) == 0) {
          printf("error: second operator is zero");
          break;
        }
        push((double) ((int) op1 % (int) op2));
        break;

      case '_':
        commandID = getcommandID();
        switch(commandID) {
          case 0: // EOF received while reading command. But needed to be pressed twice
            // printf("success exit\n");
            exit(EXIT_SUCCESS);

          case 1: // TOP
            _top();
            break;

          case 2: // SWAP
            _swap();
            break;

          case 3: // EMPTY
            _empty();
            break;

          case 4: // LET
            _let();
            break;

          case 101: // SIN
            push(sin(pop()));
            break;

          case 102: // COS
            push(cos(pop()));
            break;

          case 103: // TAN
            push(tan(pop()));
            break;

          case 111: // EXP
            push(exp(pop()));
            break;

          case 112: // POW
            op2 = pop();
            op1 = pop();
            push(pow(op1, op2));
            break;

          default:
            printf("error: command not recognized\n");
            break;
        }
        break;

      case '\n':
        // after every newline print the last element of stack as result. If
        // every operand was dealt with some suitable operator then the stack
        // will be empty by now, but if there were some unused operators(12 in
        // `12 2 3 +`) then they will be successively popped out after every
        // empty line

        // Good habit is to spam <CR> after every line of calculation to empty
        // the stack
        if(sp > 0) {
          print(Stack[sp - 1]);
        } else {
          printf("message: stack is empty, no top level element\n");
        }
        break;

      default:
        printf("error: unrecognozed character\n");
        break;
    }
  }

  return 0;
}


double pop(void) {
  // pops a double from the operator stack
  if(sp > 0) {
    return Stack[--sp];
  } else {
    printf("error: stack empty, default 0.0 returned\n");
    return 0.0;
  }
}

void push(double number) {
  // pushes a double to the operator stack
  if(sp < STACKDEPTH) {
    Stack[sp++] = number;
    return;
  } else {
    printf("error: operator stack overflow\n");
  }
}

char getch(void);
void ungetch(char c);

#define CHARBUFFSIZE 10 // character buffer size for getch() and ungetch()
char charBuffer[CHARBUFFSIZE];
int charsInBuff = 0; // count of chars in charBuffer

char getop(char s[]) { // get operator-operand
  int c, i = 0;

  // ignore spaces and tabs
  while ((c = getch()) == ' ' || c == '\t') 
    ;

  // check if c is an operator(not subtraction), newline, command or EOF. If it is then
  // return it.
  if(!isdigit(c) && c != '.' && c != '-' && c != '$') {
    return c;
  }

  // handling negative numbers(not negative of variables)
  if (c == '-') {
    if(!isdigit(c = getch()) && c != '.') {
      // means the - was an operator
      ungetch(c);
      return '-';
    } else {
      // the - was part of an operand and currently c holds a digit or decimal
      s[i++] = '-';
    }
  }

  if(c == '$') { // -$a will not be taken as the negative of the variable `a`
    if (isalpha(c = getch()) && isspace(getch())) {
      s[i++] = c;
      s[i] = '\0';
      return VARIABLE;
    } else {
      printf("error: wrong variable name, ignoring it\n");
      return '~'; // will be handled by default case
    }
  }

  // if c is a number then store it as a string in s[]
  if(isdigit(c) || c == '.') {
    ungetch(c);
    // the user needs to make sure operands have only one decimal point. If it
    // has more like 12.45.4 then the failsafe of atof() will be used and only 
    // 12.45 will be taken as the operator
    while(isdigit(c = getch()) || c == '.') {
      s[i++] = c;
    }
    s[i] = '\0';
    // the string s[] now contains the operand-string
    // signal that we got an operand
    return NUMBER;
  }

}

char getch() {
  // if characters in charBuffer is zero then return getchar() else return the
  // last character in buffer and reduce charsInBuff by one

  return (charsInBuff == 0) ? getchar() : charBuffer[--charsInBuff];
}

void ungetch(char c) {
  // store c in the charBuffer so that next getch() can retrive it.
  charBuffer[charsInBuff++] = c;
  return;
}

int getcommandID(void) {
  char command[100];
  int c, i = 0;
  while((c = getchar()) != EOF && c != '\n' && c != ' ') {
    if(isalnum(c)) {
      command[i++] = c;
    }
  }
  command[i] = '\0';

  if(c == EOF) {
    return 0;
  } else if(c == '\n') {
    // this makes sure that after executing every command <CR> is pressed 
    // and shows the stack-top
    ungetch(c);
  }

  // add other commands before library functions
  if(strcmp(command, TOP) == 0) { // TOP = 1
    return 1;
  } else if(strcmp(command, SWAP) == 0) { // SWAP = 2
    return 2;
  } else if(strcmp(command, EMPTY) == 0) { // EMPTY = 3
    return 3;
  } else if(strcmp(command, LET) == 0) {
    return 4;
  } else if(strcmp(command, SIN) == 0) { // SIN = 101
    return 101;
  } else if(strcmp(command, COS) == 0) { // COS = 102
    return 102;
  } else if(strcmp(command, TAN) == 0) { // TAN = 103
    return 103;
  } else if(strcmp(command, EXP) == 0) { // EXP = 111
    return 111;
  } else if(strcmp(command, POW) == 0) { // POW = 112
    return 112;
  } else {
    return -1; // default: unrecognized command
  }
}

void _top(void) {
  if(sp > 0) {
    printf("Stack top: %lf\n", Stack[sp - 1]);
  } else {
    printf("error: stack is empty, no top level element\n");
  }

  // because all these system commands themselves print thie required
  // infotmation so there is no need of storing the '\n' in charBuffer. But 
  // in commands like _sin etc when they are executed then only the stack 
  // is changed and they need this extra '\n' to show stacktop
  getch();
  return;
}

void _swap(void) {
  double lastLevel;
  double lastLastLevel;
  if(sp < 2) {
    printf("error: not enough levels to swap\n");
  } else {
    lastLevel = pop();
    lastLastLevel = pop();
    push(lastLevel);
    push(lastLastLevel);

    printf("[...|%lf|%lf] => [...|%lf|%lf]\n", lastLastLevel, lastLevel,
        Stack[sp - 2], Stack[sp - 1]);
  }
  getch();
  return;
}

void _empty(void) {
  if(sp > 0) {
    sp = 0;
    printf("alert: stack emptied\n");
  } else {
    printf("message: stack is already empty\n");
  }
  getch();
  return;
}

void print(double num) {
  lastPrint = num;
  printf("Stack top: %lf\n", lastPrint);
  return;
}

void _let(void) {
  int c;
  // this loops hopefully stops at first alphabet and the next character is '\n'
  while((c = getch()) == ' ' || c == '\t')
    ;

  if(isalpha(c)) { // correct variable name
    if(sp > 0) {
      variableExists[tolower(c) - 'a'] = 1;
      variables[tolower(c) - 'a'] = Stack[sp - 1];
    } else {
      printf("error: stack empty, there is nothing to put in variable %c\n", c);
    }
    // if variable is correctly registered then there will already be a '\n'
    // which will show the stack top
  } else {
    printf("error: wrong variable name\n");
    // ignore the rest of the variable name
    while((c = getch()) != ' ' && c != '\t' && c != '\n')
      ;

    if(c == '\n') ungetch(c); // to show the stack top
  }

}
