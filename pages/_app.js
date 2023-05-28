import '../styles/globals.css'
import dynamic from 'next/dynamic'         // We have to dynamically import our WalletConnectionProvider
require('@solana/wallet-adapter-react-ui/styles.css')



const WalletConnectionProvider = dynamic(                            // Here we are just importing WalletConnectionProvider (dynamically)
  () => import('../context/WalletConnectionProvider'),
  {
    ssr :false,
  }
)


function MyApp({ Component, pageProps }) {
  return(
      <WalletConnectionProvider>
        <Component {...pageProps} />
      </WalletConnectionProvider>
  )
}

export default MyApp
