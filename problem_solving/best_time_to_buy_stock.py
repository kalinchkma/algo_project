"""
#. Best Time to Buy and Sell Stock

You are given an array 'prices' where 'prices[i]' is the price of a given stock on the 'i^th' day.

You want to maximize your profit by choosing a single day to buy 
one stock and choosing a different day in the future to sell the stock.

Return the maximum profit you can achieve from this transaction.
If you cannot achieve any profit, return '0'

"""


def maxProfit(prices):
    if not prices:
        return 0

    min_price = float("inf")
    max_profit = 0

    for p in prices:
        if p < min_price:
            min_price = p
        elif p - min_price > max_profit:
            max_profit = p - min_price

    return max_profit


# Slower
def maxProfit2(prices):
    if not prices:
        return 0

    profit = 0
    min_price = float("inf")
    for p in prices:
        min_price = min(min_price, p)
        profit = max(profit, p - min_price)
    return profit


if __name__ == "__main__":
    prices = [7, 1, 5, 3, 6, 4]
    prices2 = [7, 6, 4, 3, 1]
    prices3 = [2, 4, 1]
    print(maxProfit2(prices))
    print(maxProfit2(prices2))
    print(maxProfit2(prices3))
