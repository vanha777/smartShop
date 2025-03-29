'use client'
import { Suspense } from "react";
import SimpleLoading from "./components/simpleLoading";
import DashboardClient from "./DashboardClient";
import { redirect } from "next/navigation";
import { UserData } from "../utils/AppContext";
import type { Metadata } from 'next'

// export const metadata: Metadata = {
//   title: 'CoLaunch - Connect Business and Scale Up',
//   description: 'CoLaunch is a platform that connects businesses and scale ups with the right people and resources to help them grow.',
//   openGraph: {
//     title: 'CoLaunch - Connect Business and Scale Up',
//     description: 'CoLaunch is a platform that connects businesses and scale ups with the right people and resources to help them grow.',
//     // url: 'https://www.metaloot.dev/',
//     images: [
//       {
//         url: 'https://vbfejmafjqgcfrzxewcd.supabase.co/storage/v1/object/public/general//colaunchit.jpeg',
//         alt: 'CoLaunch - Connect Business and Scale Up',
//       },
//     ],
//   },
//   icons: {
//     icon: '/logo.png',
//     // You can also specify different sizes
//     apple: [
//       { url: '/logo.png' },
//       { url: '/apple.png', sizes: '180x180' }
//     ],
//     shortcut: '/favicon.ico'
//   },
// };

export default function Dashboard({ searchParams }: { searchParams: { [key: string]: string | undefined } }) {
  // const initialUser = searchParams.user;
  // console.log("initialUser", initialUser);
  return (
    <Suspense fallback={<SimpleLoading />}>
      {/* <DashboardClient rawUser={initialUser} /> */}
      <div className="fixed inset-0 flex items-center justify-center bg-gradient-to-br from-green-900/80 to-emerald-900/80 backdrop-blur-sm z-50 animate-fadeIn">
        <div className="bg-white p-10 rounded-2xl shadow-2xl max-w-md text-center border-l-4 border-green-500 transform transition-all duration-500 animate-slideUp">
          <div className="mb-6">
            <div className="w-24 h-24 bg-green rounded-full flex items-center justify-center mx-auto mb-2">
              <svg xmlns="http://www.w3.org/2000/svg" className="h-16 w-16 text-green-500" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2.5} d="M5 13l4 4L19 7" />
              </svg>
            </div>
            <div className="h-1 w-16 bg-gradient-to-r from-green-400 to-green-600 mx-auto rounded-full"></div>
          </div>
          <h2 className="text-3xl font-bold mb-3 text-gray-800">Thank You for Registering!</h2>
          <p className="text-gray-600 mb-8 text-lg">We're excited to have you join us. We'll keep you updated with the latest news and exciting developments.</p>
          <button 
            onClick={() => window.location.href = '/'}
            className="btn w-full py-3 bg-gradient-to-r from-green to-emerald-600 hover:from-green-600 hover:to-emerald-700 text-white rounded-lg font-semibold text-lg transition-all duration-300 shadow-md hover:shadow-lg transform hover:-translate-y-1 border-none"
          >
            Return to Home
          </button>
        </div>
      </div>
    </Suspense>
  );
}

// Add these animations to your global CSS or tailwind.config.js
// @keyframes fadeIn {
//   from { opacity: 0; }
//   to { opacity: 1; }
// }
// 
// @keyframes slideUp {
//   from { opacity: 0; transform: translateY(20px); }
//   to { opacity: 1; transform: translateY(0); }
// }
// 
// .animate-fadeIn { animation: fadeIn 0.6s ease-out; }
// .animate-slideUp { animation: slideUp 0.8s ease-out; }
