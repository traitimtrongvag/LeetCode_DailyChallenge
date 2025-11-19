impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min_price = i32::MAX;
        let mut max_profit = 0;

        for price in prices {
            min_price = min_price.min(price);
            max_profit = max_profit.max(price - min_price);
        }

        max_profit
    }
}
/*
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min_price = prices[0];
        let mut max_profit = 0;

        for price in prices {
            if price < min_price {
                min_price = price;
            } else {
                let profit = price - min_price;
                if profit > max_profit {
                    max_profit = profit;
                }
            }
        }

        max_profit
    }
}	
*/