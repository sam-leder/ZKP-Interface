# Elements of the demo

## I. Explanation

* What is a ZKP?
* ...

## II. Interactive example: mortgage application

### Parties involved
* Bank
* AFM
* Consumer

### Steps in the process
#### Preparation phase
1. Bank designs and trains it model
2. AFM receives model and approves it (signs commitment to the model and publishes signature)

#### Application phase
3. Consumer sends full application A to the Bank
4. Bank runs its model M, sends back:
    * Approval X (yes/no)
    * P = proof{X = M(A)}, i.e. X is the real output of running M on A
5. Consumer verifies the proof (combining P, A, X, the commitment to M)


## Screens in the demo, by priority
1. Application (from the Consumer's perspective)
    * Fill in your details (income etc.); or maybe autofill these to save time?
    * Click "Submit" and after some seconds get back output: an approval and a proof
    * Need to verify the proof (mostly when approval is rejected) -> in another screen?

2. Proof verification (from the Consumer's perspective)
    * Pop up window summarizing all input (P, A, X), also find correct commitment
    * Click "Verify" and get back correctness yes/no
    * (If incorrect, button "contact AFM immediately")
    * **Ideally:** have a fair bank and a cheating bank, verify both

3. Commitment / signature repository hosted by the AFM (from the Consumer's perspective)
    * List of algorithms, e.g. models by Nice Bank and Mean Bank (but both are of course approved)

4. Approval process (from the Bank's perspective)
    * Upload full model (structure, weights, etc.)
    * After a while, get approval from AFM and see the commitment added to the repository

5. Inspect own model (from the Bank's perspective)
    * Visual representation of its own decision tree
