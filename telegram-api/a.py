import keyboard
import time

def simulate_key_presses(text):
    """
    Simulate typing text by pressing individual keys
    """
    # Wait a moment before starting (optional)
    time.sleep(5)

    # Type the text
    keyboard.write(text)

# Example usage
text = """
As our company is in process of updating our authorized signatories and I need some information on the process for changing the founder and authorized representative on our account.

To help us prepare, could you please clarify a few points for me?

1. Required Documents: What is the complete list of documents needed for this change? Please include any requirements for both the outgoing and incoming person, as well as any company documents.

2. Branch Visit: Who needs to come to the branch in person? Is it only the new founder/authorized person, or does the previous representative also need to be present?

3. Timing and Forms: How long does this process usually take once all documents are submitted? Also, are there specific bank forms we need to fill out to authorize the new person?


Thank you for your help.
"""
simulate_key_presses(text)
