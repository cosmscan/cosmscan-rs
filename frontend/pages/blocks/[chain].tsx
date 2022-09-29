import type { NextPage } from "next";
import Layout from "../../components/layout";

const ListBlockPage:NextPage = () => {
    return (
        <Layout>
            <div className="mx-auto w-10/12 px-2 mt-16">
                <div className="sm:flex sm:items-center">
                    <h1 className="text-4xl font-semibold">Recent Blocks</h1>
                </div>
                <div className="mt-8 flex flex-col">
                    <div className="-my-2 -mx-4 overflow-x-auto sm:-mx-6 lg:-mx-8">
                    <div className="inline-block min-w-full py-2 align-middle md:px-6 lg:px-8">
                        <div className="overflow-hidden shadow ring-1 ring-black ring-opacity-5 md:rounded-lg">
                        <table className="min-w-full divide-y divide-gray-500 ">
                            <thead className="bg-gray-800">
                            <tr>
                                <th scope="col" className="py-3.5 pl-4 text-left font-semibold sm:pl-6">Height</th>
                                <th scope="col" className="px-3 py-3.5 text-left font-semibold">Block Hash</th>
                                <th scope="col" className="px-3 py-3.5 text-left font-semibold">Time</th>
                                <th scope="col" className="px-3 py-3.5 text-left font-semibold">Tx Count</th>
                                <th scope="col" className="px-3 py-3.5 text-left font-semibold">Proposer</th>
                            </tr>
                            </thead>
                            <tbody className="divide-y divide-gray-500 bg-gray-700">
                                <tr key={"1"}>
                                    <td className="whitespace-nowrap py-4 pl-4 pr-3 font-medium sm:pl-6">12141761</td>
                                    <td className="whitespace-nowrap px-3 py-4">647B1266BC9D607EFFD5C0E93C81D7619D3F443D5E9BE703A50C7A76620B1739</td>
                                    <td className="whitespace-nowrap px-3 py-4">7 seconds ago</td>
                                    <td className="whitespace-nowrap px-3 py-4">100</td>
                                    <td className="whitespace-nowrap px-3 py-4">Figment</td>
                                </tr>
                                <tr key={"2"}>
                                    <td className="whitespace-nowrap py-4 pl-4 pr-3 font-medium sm:pl-6">12141761</td>
                                    <td className="whitespace-nowrap px-3 py-4">647B1266BC9D607EFFD5C0E93C81D7619D3F443D5E9BE703A50C7A76620B1739</td>
                                    <td className="whitespace-nowrap px-3 py-4">7 seconds ago</td>
                                    <td className="whitespace-nowrap px-3 py-4">100</td>
                                    <td className="whitespace-nowrap px-3 py-4">Figment</td>
                                </tr>
                                <tr key={"3"}>
                                    <td className="whitespace-nowrap py-4 pl-4 pr-3 font-medium sm:pl-6">12141761</td>
                                    <td className="whitespace-nowrap px-3 py-4">647B1266BC9D607EFFD5C0E93C81D7619D3F443D5E9BE703A50C7A76620B1739</td>
                                    <td className="whitespace-nowrap px-3 py-4">7 seconds ago</td>
                                    <td className="whitespace-nowrap px-3 py-4">100</td>
                                    <td className="whitespace-nowrap px-3 py-4">Figment</td>
                                </tr>
                                <tr key={"4"}>
                                    <td className="whitespace-nowrap py-4 pl-4 pr-3 font-medium sm:pl-6">12141761</td>
                                    <td className="whitespace-nowrap px-3 py-4">647B1266BC9D607EFFD5C0E93C81D7619D3F443D5E9BE703A50C7A76620B1739</td>
                                    <td className="whitespace-nowrap px-3 py-4">7 seconds ago</td>
                                    <td className="whitespace-nowrap px-3 py-4">100</td>
                                    <td className="whitespace-nowrap px-3 py-4">Figment</td>
                                </tr>
                            </tbody>
                        </table>
                        </div>
                    </div>
                    </div>
                </div>
            </div>
        </Layout>
    )
}

export default ListBlockPage;