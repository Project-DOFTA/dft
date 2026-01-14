use near_sdk::borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::json_types::U128;
use near_sdk::{env, near, AccountId, Balance, PanicOnDefault, Promise};

/// Status of an escrow order
#[near(serializers = [json, borsh])]
#[derive(Clone, Debug, PartialEq)]
pub enum OrderStatus {
    Pending,
    Completed,
    Refunded,
    Disputed,
    Resolved,
}

/// Resolution decision for a dispute
#[near(serializers = [json, borsh])]
#[derive(Clone, Debug, PartialEq)]
pub enum Resolution {
    RefundBuyer,
    PaySeller,
}

/// Escrow order structure
#[near(serializers = [json, borsh])]
#[derive(Clone)]
pub struct EscrowOrder {
    pub order_id: String,
    pub buyer: AccountId,
    pub seller: AccountId,
    pub amount: Balance,
    pub listing_id: String,
    pub quantity: u32,
    pub status: OrderStatus,
    pub created_at: u64,
    pub completed_at: Option<u64>,
}

/// Main marketplace contract
#[near(contract_state)]
#[derive(PanicOnDefault)]
pub struct MarketplaceContract {
    pub owner: AccountId,
    pub orders: UnorderedMap<String, EscrowOrder>,
    pub platform_fee_percentage: u8, // e.g., 2 for 2%
}

#[near]
impl MarketplaceContract {
    /// Initialize the contract
    #[init]
    pub fn new(owner: AccountId, platform_fee_percentage: u8) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        assert!(
            platform_fee_percentage <= 10,
            "Platform fee cannot exceed 10%"
        );
        
        Self {
            owner,
            orders: UnorderedMap::new(b"o"),
            platform_fee_percentage,
        }
    }

    /// Create an escrow order (buyer deposits funds)
    #[payable]
    pub fn create_order(
        &mut self,
        order_id: String,
        seller: AccountId,
        listing_id: String,
        quantity: u32,
    ) -> EscrowOrder {
        let buyer = env::predecessor_account_id();
        let amount = env::attached_deposit();

        // Validate inputs
        assert!(amount > 0, "Must attach NEAR tokens");
        assert!(quantity > 0, "Quantity must be greater than 0");
        assert!(
            !self.orders.get(&order_id).is_some(),
            "Order ID already exists"
        );
        assert!(buyer != seller, "Buyer and seller must be different");

        // Create escrow order
        let order = EscrowOrder {
            order_id: order_id.clone(),
            buyer: buyer.clone(),
            seller,
            amount,
            listing_id,
            quantity,
            status: OrderStatus::Pending,
            created_at: env::block_timestamp(),
            completed_at: None,
        };

        self.orders.insert(&order_id, &order);

        env::log_str(&format!(
            "Escrow created: {} - Buyer: {} - Amount: {} yoctoNEAR",
            order_id, buyer, amount
        ));

        order
    }

    /// Complete order and release funds to seller (called by buyer)
    pub fn complete_order(&mut self, order_id: String) {
        let caller = env::predecessor_account_id();
        let mut order = self
            .orders
            .get(&order_id)
            .expect("Order not found");

        // Validate
        assert_eq!(order.buyer, caller, "Only buyer can complete order");
        assert_eq!(order.status, OrderStatus::Pending, "Order not pending");

        // Calculate platform fee and seller amount
        let platform_fee = (order.amount * self.platform_fee_percentage as u128) / 100;
        let seller_amount = order.amount - platform_fee;

        // Update order status
        order.status = OrderStatus::Completed;
        order.completed_at = Some(env::block_timestamp());
        self.orders.insert(&order_id, &order);

        // Transfer funds
        if platform_fee > 0 {
            Promise::new(self.owner.clone()).transfer(platform_fee);
        }
        Promise::new(order.seller.clone()).transfer(seller_amount);

        env::log_str(&format!(
            "Order completed: {} - Seller received: {} yoctoNEAR - Platform fee: {} yoctoNEAR",
            order_id, seller_amount, platform_fee
        ));
    }

    /// Refund order (called by seller or owner in case of dispute)
    pub fn refund_order(&mut self, order_id: String) {
        let caller = env::predecessor_account_id();
        let mut order = self
            .orders
            .get(&order_id)
            .expect("Order not found");

        // Validate
        assert_eq!(order.status, OrderStatus::Pending, "Order not pending");
        assert!(
            caller == order.seller || caller == self.owner,
            "Only seller or owner can refund"
        );

        // Update order status
        order.status = OrderStatus::Refunded;
        order.completed_at = Some(env::block_timestamp());
        self.orders.insert(&order_id, &order);

        // Refund buyer
        Promise::new(order.buyer.clone()).transfer(order.amount);

        env::log_str(&format!(
            "Order refunded: {} - Buyer refunded: {} yoctoNEAR",
            order_id, order.amount
        ));
    }

    pub fn dispute_order(&mut self, order_id: String) {
        let caller = env::predecessor_account_id();
        let mut order = self
            .orders
            .get(&order_id)
            .expect("Order not found");

        // Validate
        assert_eq!(order.status, OrderStatus::Pending, "Order not pending");
        assert!(
            caller == order.buyer || caller == order.seller,
            "Only buyer or seller can dispute"
        );

        // Update order status
        order.status = OrderStatus::Disputed;
        self.orders.insert(&order_id, &order);

        env::log_str(&format!(
            "Order disputed: {} - Awaiting owner resolution",
            order_id
        ));
    }

    /// Resolve a disputed order (owner only)
    pub fn resolve_dispute(&mut self, order_id: String, resolution: Resolution) {
        let caller = env::predecessor_account_id();
        assert_eq!(caller, self.owner, "Only owner can resolve disputes");

        let mut order = self
            .orders
            .get(&order_id)
            .expect("Order not found");

        assert_eq!(
            order.status,
            OrderStatus::Disputed,
            "Order is not disputed"
        );

        match resolution {
            Resolution::RefundBuyer => {
                // Refund buyer in full
                Promise::new(order.buyer.clone()).transfer(order.amount);
                env::log_str(&format!(
                    "Dispute resolved for {}: Buyer refunded {} yoctoNEAR",
                    order_id, order.amount
                ));
            }
            Resolution::PaySeller => {
                // Pay seller (minus platform fee)
                let platform_fee = (order.amount * self.platform_fee_percentage as u128) / 100;
                let seller_amount = order.amount - platform_fee;

                if platform_fee > 0 {
                    Promise::new(self.owner.clone()).transfer(platform_fee);
                }
                Promise::new(order.seller.clone()).transfer(seller_amount);

                env::log_str(&format!(
                    "Dispute resolved for {}: Seller paid {} yoctoNEAR",
                    order_id, seller_amount
                ));
            }
        }

        order.status = OrderStatus::Resolved;
        order.completed_at = Some(env::block_timestamp());
        self.orders.insert(&order_id, &order);
    }

    /// Get order details
    pub fn get_order(&self, order_id: String) -> Option<EscrowOrder> {
        self.orders.get(&order_id)
    }

    /// Get all orders for a buyer
    pub fn get_buyer_orders(&self, buyer: AccountId) -> Vec<EscrowOrder> {
        self.orders
            .values()
            .filter(|order| order.buyer == buyer)
            .collect()
    }

    /// Get all orders for a seller
    pub fn get_seller_orders(&self, seller: AccountId) -> Vec<EscrowOrder> {
        self.orders
            .values()
            .filter(|order| order.seller == seller)
            .collect()
    }

    /// Get platform fee percentage
    pub fn get_platform_fee(&self) -> u8 {
        self.platform_fee_percentage
    }

    /// Update platform fee (owner only)
    pub fn update_platform_fee(&mut self, new_fee: u8) {
        assert_eq!(
            env::predecessor_account_id(),
            self.owner,
            "Only owner can update fee"
        );
        assert!(new_fee <= 10, "Fee cannot exceed 10%");
        
        self.platform_fee_percentage = new_fee;
        env::log_str(&format!("Platform fee updated to {}%", new_fee));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::testing_env;

    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .predecessor_account_id(predecessor)
            .block_timestamp(1_000_000_000);
        builder
    }

    #[test]
    fn test_contract_initialization() {
        let context = get_context(accounts(0));
        testing_env!(context.build());
        
        let contract = MarketplaceContract::new(accounts(0), 2);
        assert_eq!(contract.owner, accounts(0));
        assert_eq!(contract.platform_fee_percentage, 2);
    }

    #[test]
    fn test_create_order() {
        let mut context = get_context(accounts(1));
        context.attached_deposit(1_000_000_000_000_000_000_000_000); // 1 NEAR
        testing_env!(context.build());

        let mut contract = MarketplaceContract::new(accounts(0), 2);
        
        let order = contract.create_order(
            "order_1".to_string(),
            accounts(2),
            "listing_1".to_string(),
            5,
        );

        assert_eq!(order.buyer, accounts(1));
        assert_eq!(order.seller, accounts(2));
        assert_eq!(order.status, OrderStatus::Pending);
    }

    #[test]
    #[should_panic(expected = "Must attach NEAR tokens")]
    fn test_create_order_no_deposit() {
        let context = get_context(accounts(1));
        testing_env!(context.build());

        let mut contract = MarketplaceContract::new(accounts(0), 2);
        
        contract.create_order(
            "order_1".to_string(),
            accounts(2),
            "listing_1".to_string(),
            5,
        );
    }

    #[test]
    fn test_resolve_dispute_refund_buyer() {
        let mut context = get_context(accounts(0)); // Owner
        testing_env!(context.build());
        let mut contract = MarketplaceContract::new(accounts(0), 2);

        // Setup: Create order and dispute it
        // Buyer creates order
        context.predecessor_account_id(accounts(1)); // Buyer
        context.attached_deposit(1_000_000_000_000_000_000_000_000);
        testing_env!(context.build());
        contract.create_order("o1".to_string(), accounts(2), "l1".to_string(), 1);

        // Buyer disputes order (can be buyer or seller)
        contract.dispute_order("o1".to_string());

        // Test: Owner resolves dispute (Refund Buyer)
        context.predecessor_account_id(accounts(0)); // Back to owner
        context.attached_deposit(0);
        testing_env!(context.build());

        contract.resolve_dispute("o1".to_string(), Resolution::RefundBuyer);

        let order = contract.get_order("o1".to_string()).unwrap();
        assert_eq!(order.status, OrderStatus::Resolved);
    }

    #[test]
    fn test_resolve_dispute_pay_seller() {
        let mut context = get_context(accounts(0));
        testing_env!(context.build());
        let mut contract = MarketplaceContract::new(accounts(0), 2);

        // Setup
        context.predecessor_account_id(accounts(1));
        context.attached_deposit(1_000_000_000_000_000_000_000_000);
        testing_env!(context.build());
        contract.create_order("o2".to_string(), accounts(2), "l2".to_string(), 1);
        
        // Seller disputes
        context.predecessor_account_id(accounts(2));
        testing_env!(context.build());
        contract.dispute_order("o2".to_string());

        // Owner resolves
        context.predecessor_account_id(accounts(0));
        testing_env!(context.build());
        contract.resolve_dispute("o2".to_string(), Resolution::PaySeller);

        let order = contract.get_order("o2".to_string()).unwrap();
        assert_eq!(order.status, OrderStatus::Resolved);
    }

    #[test]
    #[should_panic(expected = "Only owner can resolve disputes")]
    fn test_resolve_dispute_unauthorized() {
        let mut context = get_context(accounts(0));
        testing_env!(context.build());
        let mut contract = MarketplaceContract::new(accounts(0), 2);

        context.predecessor_account_id(accounts(1));
        context.attached_deposit(1_000_000_000_000_000_000_000_000);
        testing_env!(context.build());
        contract.create_order("o3".to_string(), accounts(2), "l3".to_string(), 1);
        
        // Dispute
        contract.dispute_order("o3".to_string());

        // Attacker tries to resolve
        context.predecessor_account_id(accounts(3));
        testing_env!(context.build());
        contract.resolve_dispute("o3".to_string(), Resolution::RefundBuyer);
    }
}
