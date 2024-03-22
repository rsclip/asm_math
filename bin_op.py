from typing import *

def add(s: str, t: str) -> Tuple[str]:
    """
    Add two binary numbers together.

    Args:
        s: A binary number.
        t: Another binary number.

    Returns:
        (
            str: lower bits of the sum,
            str: higher bits (if overflow)
        )
    """
    result = ""
    carry = 0
    
    # Start from the rightmost digit (least significant bit)
    i = len(s) - 1
    j = len(t) - 1
    
    while i >= 0 or j >= 0:
        # Get the current bits, or assume 0 if we've reached the end of a number
        bit_s = int(s[i]) if i >= 0 else 0
        bit_t = int(t[j]) if j >= 0 else 0
        
        # Calculate the sum of current bits and the carry
        current_sum = bit_s + bit_t + carry
        
        # Append the lower bit of the sum to the result
        result = str(current_sum % 2) + result
        
        # Update the carry for the next iteration
        carry = current_sum // 2
        
        # Move to the next bit
        i -= 1
        j -= 1
    
    # If there's still a carry left, append it as the higher bit
    if carry:
        result = '1' + result
        
    # Split the result into lower and higher bits
    lower_bits = result[-len(s):]
    higher_bits = result[:-len(s)] if len(result) > len(s) else ""
    
    return lower_bits, higher_bits

U64MAX = "1111111111111111111111111111111111111111111111111111111111111111"

low, hi = add(U64MAX, U64MAX)
print(f"low: {low}, hi: {hi}")