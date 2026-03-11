int bitwiseComplement(int n)
{
    if(n == 0) return 1;

    int result = 0;
    int power = 1;

    while(n > 0)
    {
        int bit = n % 2;     // get the last bit

        if(bit == 0)
            result += power; // flip 0 -> 1

        n /= 2;              // remove the last bit
        power *= 2;          // move to the next bit position
    }

    return result;
}
