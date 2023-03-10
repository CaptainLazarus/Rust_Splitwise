## Algorithm

```
1. Calculate the total flow of each node.

2. Seperate the nodes into Seeders (those with '+'ve flow) and Leechers (those with '-'ve flow)

3. Match head values of both lists. If

4. seeder(head.value) **>** leecher(head.value)
    a. pop leecher head.
    b. subtract leecher value from seeder.
    c. Goto 3

5. seeder(head.value) **<** leecher(head.value)
    a. pop seeder head.
    b. subtract seeder value from leecher.
    c. Goto 3

6. If equal
    a. Pop both leecher and seeder.
    b. Goto 3

7. Repeat until both lists are empty.

Emptiness is guaranteed since the total value of both lists are equal (difference in sign).
```