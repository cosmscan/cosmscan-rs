import { ArrowLongLeftIcon, ArrowLongRightIcon, ChevronLeftIcon, ChevronRightIcon } from "@heroicons/react/20/solid";
import type { NextPage } from "next";
import Layout from "../../components/layout";

const ListTransactionPage: NextPage = () => {
    return (
        <Layout>
            <div className="mx-auto w-10/12 px-2 mt-16">
                <div className="sm:items-center">
                    <h1 className="text-4xl font-semibold">Transactions</h1>
                    <p className="text-xl pt-3 text-gray-400">
                        Total 1,718,533,059 transactions found
                    </p>
                </div>
                <div className="mt-8 flex flex-col">
                    <div className="-my-2 -mx-4 overflow-x-auto sm:-mx-6 lg:-mx-8">
                        <div className="inline-block min-w-full py-2 align-middle md:px-6 lg:px-8">
                            <div className="overflow-hidden shadow ring-opacity-5 md:rounded-lg">
                                <table className="min-w-full divide-y divide-gray-500 ">
                                    <thead className="bg-gray-800">
                                        <tr>
                                            <th scope="col" className="py-3.5 pl-4 text-left font-semibold sm:pl-6">Tx Hash</th>
                                            <th scope="col" className="px-3 py-3.5 text-left font-semibold">Type</th>
                                            <th scope="col" className="px-3 py-3.5 text-left font-semibold">Result</th>
                                            <th scope="col" className="px-3 py-3.5 text-left font-semibold">Height</th>
                                            <th scope="col" className="px-3 py-3.5 text-left font-semibold">Amount</th>
                                            <th scope="col" className="px-3 py-3.5 text-left font-semibold">Fee</th>
                                            <th scope="col" className="px-3 py-3.5 text-left font-semibold">Time</th>
                                        </tr>
                                    </thead>
                                    <tbody className="divide-y divide-gray-500 bg-gray-700">
                                        <tr key={"1"}>
                                            <td className="whitespace-nowrap py-4 pl-4 pr-3 font-medium sm:pl-6">12141761</td>
                                            <td className="whitespace-nowrap px-3 py-4">647B1266BC9D607EFFD5C0E93C81D7619D3F443D5E9BE703A50C7A76620B1739</td>
                                            <td className="whitespace-nowrap px-3 py-4">7 seconds ago</td>
                                            <td className="whitespace-nowrap px-3 py-4">100</td>
                                            <td className="whitespace-nowrap px-3 py-4">Figment</td>
                                            <td className="whitespace-nowrap px-3 py-4">100</td>
                                            <td className="whitespace-nowrap px-3 py-4">Figment</td>
                                        </tr>
                                    </tbody>
                                </table>
                            </div>
                        </div>
                    </div>
                    <div className="mt-5 text-end">
                        <nav className="isolate inline-flex -space-x-px rounded-md shadow-sm" aria-label="Pagination">
                            <a
                                href="#"
                                className="relative inline-flex items-center rounded-l-md border border-gray-600 px-2 py-2 text-sm font-medium text-gray-300 hover:bg-gray-600 focus:z-20"
                            >
                                First
                            </a>
                            <a
                                href="#"
                                className="relative inline-flex items-center border border-gray-600 px-2 py-2 text-sm font-medium text-gray-300 hover:bg-gray-600 focus:z-20"
                            >
                                <span className="sr-only">Previous</span>
                                <ChevronLeftIcon className="h-5 w-5" aria-hidden="true" />
                            </a>
                            {/* Current: "z-10 bg-indigo-50 border-indigo-500 text-indigo-600", Default: "border-gray-600 text-gray-300 hover:bg-gray-600" */}
                            <a
                                href="#"
                                aria-current="page"
                                className="relative z-10 inline-flex items-center border border-indigo-500 bg-indigo-400 px-4 py-2 text-sm font-medium text-indigo-600 focus:z-20"
                            >
                                1
                            </a>
                            <a
                                href="#"
                                className="relative inline-flex items-center border border-gray-600 px-4 py-2 text-sm font-medium text-gray-300 hover:bg-gray-600 focus:z-20"
                            >
                                2
                            </a>
                            <a
                                href="#"
                                className="relative inline-flex items-center border border-gray-600 px-4 py-2 text-sm font-medium text-gray-300 hover:bg-gray-600 focus:z-20"
                            >
                                2
                            </a>
                            <a
                                href="#"
                                className="relative inline-flex items-center border border-gray-600 px-4 py-2 text-sm font-medium text-gray-300 hover:bg-gray-600 focus:z-20"
                            >
                                2
                            </a>
                            <a
                                href="#"
                                className="relative inline-flex items-center border border-gray-600 px-4 py-2 text-sm font-medium text-gray-300 hover:bg-gray-600 focus:z-20"
                            >
                                2
                            </a>
                            <a
                                href="#"
                                className="relative inline-flex items-center border border-gray-600 px-4 py-2 text-sm font-medium text-gray-300 hover:bg-gray-600 focus:z-20"
                            >
                                2
                            </a>
                            <a
                                href="#"
                                className="relative inline-flex items-center border border-gray-600 px-2 py-2 text-sm font-medium text-gray-300 hover:bg-gray-600 focus:z-20"
                            >
                                <span className="sr-only">Next</span>
                                <ChevronRightIcon className="h-5 w-5" aria-hidden="true" />
                            </a>
                            <a
                                href="#"
                                className="relative inline-flex items-center rounded-r-md border border-gray-600 px-2 py-2 text-sm font-medium text-gray-300 hover:bg-gray-600 focus:z-20"
                            >
                                Last
                            </a>
                        </nav>
                    </div>
                </div>
            </div>
        </Layout>
    )
}

export default ListTransactionPage;