import { NearWalletProvider, useNear } from './contexts/NearWalletProvider';
import './App.css';

function AppContent() {
  const { modal, accountId, isSignedIn } = useNear();

  const handleSignIn = () => {
    modal?.show();
  };

  const handleSignOut = async () => {
    const wallet = await modal?.hide();
    // Wallet selector handles sign out
  };

  return (
    <div className="min-h-screen bg-gradient-to-br from-purple-900 via-blue-900 to-indigo-900">
      <nav className="bg-black bg-opacity-30 backdrop-blur-md border-b border-white border-opacity-10">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="flex justify-between items-center h-16">
            <div className="flex items-center">
              <h1 className="text-2xl font-bold text-white">
                🌾 DOFTA
              </h1>
              <span className="ml-3 text-sm text-gray-300">
                Decentralized Organic Farmers Trading Alliance
              </span>
            </div>
            <div>
              {isSignedIn ? (
                <div className="flex items-center gap-4">
                  <span className="text-white text-sm">{accountId}</span>
                  <button
                    onClick={handleSignOut}
                    className="bg-red-600 hover:bg-red-700 text-white px-4 py-2 rounded-lg transition-colors"
                  >
                    Sign Out
                  </button>
                </div>
              ) : (
                <button
                  onClick={handleSignIn}
                  className="bg-gradient-to-r from-green-500 to-blue-500 hover:from-green-600 hover:to-blue-600 text-white px-6 py-2 rounded-lg font-semibold transition-all"
                >
                  Connect Wallet
                </button>
              )}
            </div>
          </div>
        </div>
      </nav>

      <main className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
        <div className="text-center mb-12">
          <h2 className="text-5xl font-extrabold text-white mb-4">
            Welcome to DOFTA Marketplace
          </h2>
          <p className="text-xl text-gray-300 max-w-3xl mx-auto">
            A Web3-powered cooperative platform connecting organic farmers directly with buyers.
            Trade transparently, securely, and fairly on NEAR blockchain.
          </p>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-3 gap-8 mt-16">
          <div className="bg-white bg-opacity-10 backdrop-blur-lg rounded-2xl p-8 border border-white border-opacity-20 hover:bg-opacity-20 transition-all duration-300">
            <div className="text-4xl mb-4">🌱</div>
            <h3 className="text-2xl font-bold text-white mb-3">List Products</h3>
            <p className="text-gray-300">
              Farmers can list their organic produce with transparent pricing and availability.
            </p>
          </div>

          <div className="bg-white bg-opacity-10 backdrop-blur-lg rounded-2xl p-8 border border-white border-opacity-20 hover:bg-opacity-20 transition-all duration-300">
            <div className="text-4xl mb-4">🛒</div>
            <h3 className="text-2xl font-bold text-white mb-3">Place Orders</h3>
            <p className="text-gray-300">
              Browse products and place orders with secure escrow-based payments on NEAR.
            </p>
          </div>

          <div className="bg-white bg-opacity-10 backdrop-blur-lg rounded-2xl p-8 border border-white border-opacity-20 hover:bg-opacity-20 transition-all duration-300">
            <div className="text-4xl mb-4">🔐</div>
            <h3 className="text-2xl font-bold text-white mb-3">Secure Transactions</h3>
            <p className="text-gray-300">
              All transactions recorded on NEAR blockchain with full transparency and escrow protection.
            </p>
          </div>
        </div>

        <div className="mt-16 text-center">
          <div className="bg-gradient-to-r from-green-500 to-blue-500 rounded-2xl p-8 text-white">
            <h3 className="text-3xl font-bold mb-4">
              {isSignedIn ? `Welcome, ${accountId}!` : 'Connect Your Wallet to Get Started'}
            </h3>
            <p className="text-lg mb-6">
              {isSignedIn
                ? 'Start exploring the marketplace and trade with farmers'
                : 'Use NEAR Wallet to access the decentralized marketplace'
              }
            </p>
            {!isSignedIn && (
              <button
                onClick={handleSignIn}
                className="bg-white text-blue-600 px-8 py-3 rounded-lg font-bold text-lg hover:bg-gray-100 transition-colors"
              >
                Connect NEAR Wallet
              </button>
            )}
          </div>
        </div>
      </main>

      <footer className="bg-black bg-opacity-30 backdrop-blur-md border-t border-white border-opacity-10 mt-20">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
          <p className="text-center text-gray-400">
            © 2026 DOFTA - Empowering Organic Farmers Through Web3 on NEAR Protocol
          </p>
        </div>
      </footer>
    </div>
  );
}

function App() {
  return (
    <NearWalletProvider>
      <AppContent />
    </NearWalletProvider>
  );
}

export default App;
