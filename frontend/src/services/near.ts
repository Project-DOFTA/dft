import { connect, keyStores, Contract, WalletConnection } from 'near-api-js';

const CONTRACT_ID = import.meta.env.VITE_NEAR_CONTRACT_ID || 'dofta-marketplace.testnet';
const NETWORK_ID = import.meta.env.VITE_NEAR_NETWORK || 'testnet';

// Initialize NEAR connection
export const initNear = async () => {
    const near = await connect({
        networkId: NETWORK_ID,
        keyStore: new keyStores.BrowserLocalStorageKeyStore(),
        nodeUrl: `https://rpc.${NETWORK_ID}.near.org`,
        walletUrl: `https://wallet.${NETWORK_ID}.near.org`,
        helperUrl: `https://helper.${NETWORK_ID}.near.org`,
    });

    const wallet = new WalletConnection(near, 'dofta-marketplace');

    return { near, wallet };
};

// Contract interface
export interface EscrowOrder {
    order_id: string;
    buyer: string;
    seller: string;
    amount: string;
    listing_id: string;
    quantity: number;
    status: 'Pending' | 'Completed' | 'Refunded' | 'Disputed';
    created_at: string;
    completed_at?: string;
}

// NEAR contract methods
export class MarketplaceContract {
    private contract: Contract;

    constructor(wallet: WalletConnection) {
        this.contract = new Contract(wallet.account(), CONTRACT_ID, {
            viewMethods: ['get_order', 'get_buyer_orders', 'get_seller_orders', 'get_platform_fee'],
            changeMethods: ['create_order', 'complete_order', 'refund_order', 'dispute_order'],
        }) as any;
    }

    // Create an escrow order
    async createOrder(
        orderId: string,
        seller: string,
        listingId: string,
        quantity: number,
        amountInNear: string
    ): Promise<EscrowOrder> {
        return await (this.contract as any).create_order(
            {
                order_id: orderId,
                seller,
                listing_id: listingId,
                quantity,
            },
            '300000000000000', // gas
            amountInNear // attached NEAR
        );
    }

    // Complete order (buyer confirms delivery)
    async completeOrder(orderId: string): Promise<void> {
        return await (this.contract as any).complete_order(
            { order_id: orderId },
            '300000000000000'
        );
    }

    // Refund order (seller or owner)
    async refundOrder(orderId: string): Promise<void> {
        return await (this.contract as any).refund_order(
            { order_id: orderId },
            '300000000000000'
        );
    }

    // Dispute order
    async disputeOrder(orderId: string): Promise<void> {
        return await (this.contract as any).dispute_order(
            { order_id: orderId },
            '300000000000000'
        );
    }

    // Get order details
    async getOrder(orderId: string): Promise<EscrowOrder | null> {
        return await (this.contract as any).get_order({ order_id: orderId });
    }

    // Get buyer orders
    async getBuyerOrders(buyer: string): Promise<EscrowOrder[]> {
        return await (this.contract as any).get_buyer_orders({ buyer });
    }

    // Get seller orders
    async getSellerOrders(seller: string): Promise<EscrowOrder[]> {
        return await (this.contract as any).get_seller_orders({ seller });
    }

    // Get platform fee
    async getPlatformFee(): Promise<number> {
        return await (this.contract as any).get_platform_fee();
    }
}

// Helper to convert NEAR to yoctoNEAR
export const nearToYocto = (amount: string): string => {
    return (parseFloat(amount) * 1e24).toString();
};

// Helper to convert yoctoNEAR to NEAR
export const yoctoToNear = (amount: string): string => {
    return (parseFloat(amount) / 1e24).toFixed(4);
};
