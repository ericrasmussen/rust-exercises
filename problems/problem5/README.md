# Problem 5

`problem5` involves calculating various values relating to a mortgage:

* Joelle borrows big money sum _P_ from a bank at an interest rate of _R_ %.
* At the end of each month the debt is increased by _R_ / 12 percent.
* At the end of each month Joelle gives the bank some predefined 
payment to decrease the debt.
* The debt is considered settled when its value is reduced to zero.

The compound interest formula calculates the amount of interest 
accrued on the principal _P_. For an annual interest rate of _R_, the 
amount of interest _I_ after one month is:

    I = P * R / 12
    
The sum _P + I_ is then used as the new principal for calculating 
interest owed the following month). 

Using the struct `Mortgage` provided in `lib.rs`, which should be
initialized with a principal and interest rate:

1. Write a test for the compound interest formula, which calculates 
   the new principal after a month has passed.
2. Calculate the balance remaining on the loan after _n_ months.
3. Let's say you want to use the maximum amount of time 
(`Mortgage.loan_term_months`) to pay off the loan. Calculate the 
minimum monthly payment required to meet this goal.
4. Calculate the total amount you've paid (payments + interest) 
toward the loan after _n_ months.
5. Given an alternative monthly payment of _M_ dollars (let's say you
want to pay more than the minimum monthly payment), calculate the new
amount of time needed to pay off the loan in full.

## lib.rs

The method definitions are all empty, but there are tests for
checking your solutions. 

**Note** the test for #4, `test_mortgage_total_payout`, may fail 
due to slight rounding errors: if your resulting total payout is
off the expected value by less than $100, consider your solution 
more or less correct. (Then again, it's entirely possible that my
solution is wrong :P)

Run `cargo test --package problem5` to run the tests.

## Docs relevant to this problem

* https://doc.rust-lang.org/book/ch05-00-structs.html
* This exercise is based on [this problem](https://www.codeabbey.com/index/task_view/mortgage-calculator)
