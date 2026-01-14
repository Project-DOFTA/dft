import React, { createContext, useContext, useEffect, useState, ReactNode } from 'react';
import { setupWalletSelector, WalletSelector } from '@near-wallet-selector/core';
import { setupModal, WalletSelectorModal } from '@near-wallet-selector/modal-ui';
import { setupMyNearWallet } from '@near-wallet-selector/my-near-wallet';
import '@near-wallet-selector/modal-ui/styles.css';

interface NearContextType {
    selector: WalletSelector | null;
    modal: WalletSelectorModal | null;
    accountId: string | null;
    isSignedIn: boolean;
}

const NearContext = createContext<NearContextType>({
    selector: null,
    modal: null,
    accountId: null,
    isSignedIn: false,
});

export const useNear = () => useContext(NearContext);

interface NearWalletProviderProps {
    children: ReactNode;
}

export const NearWalletProvider: React.FC<NearWalletProviderProps> = ({ children }) => {
    const [selector, setSelector] = useState<WalletSelector | null>(null);
    const [modal, setModal] = useState<WalletSelectorModal | null>(null);
    const [accountId, setAccountId] = useState<string | null>(null);

    useEffect(() => {
        const initNear = async () => {
            const _selector = await setupWalletSelector({
                network: 'testnet',
                modules: [setupMyNearWallet()],
            });

            const _modal = setupModal(_selector, {
                contractId: import.meta.env.VITE_NEAR_CONTRACT_ID || 'dofta-marketplace.testnet',
            });

            setSelector(_selector);
            setModal(_modal);

            // Check if already signed in
            const state = _selector.store.getState();
            const accounts = state.accounts;
            if (accounts.length > 0) {
                setAccountId(accounts[0].accountId);
            }

            // Subscribe to account changes
            _selector.store.observable.subscribe((state) => {
                const accounts = state.accounts;
                setAccountId(accounts.length > 0 ? accounts[0].accountId : null);
            });
        };

        initNear();
    }, []);

    const value: NearContextType = {
        selector,
        modal,
        accountId,
        isSignedIn: !!accountId,
    };

    return <NearContext.Provider value={value}>{children}</NearContext.Provider>;
};
