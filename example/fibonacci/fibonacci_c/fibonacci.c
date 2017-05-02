int fibonacci(int number) {
  int a = 0;
  int b = 1;

  for (int i = 0; i < number; i++) {
    int tmp = a;
    a = b;
    b = a + tmp;
  }

  return a;
}
