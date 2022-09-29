const TransactionSubList = () => {
    return (
        <div className="overflow-hidden bg-gray-800 shadow sm:rounded-lg mt-5">
            <div className="px-4 py-5 sm:px-6">
                <h3 className="text-lg font-medium leading-6">Transactions</h3>
            </div>
            <div>
                <table className="min-w-full divide-y divide-gray-500 ">
                    <thead className="bg-gray-800">
                        <tr>
                            <th scope="col" className="py-3.5 pl-4 text-left text-gray-400 font-semibold sm:pl-6">Tx Hash</th>
                            <th scope="col" className="px-3 py-3.5 text-left text-gray-400 font-semibold">Type</th>
                            <th scope="col" className="px-3 py-3.5 text-left text-gray-400 font-semibold">Result</th>
                            <th scope="col" className="px-3 py-3.5 text-left text-gray-400 font-semibold">Height</th>
                            <th scope="col" className="px-3 py-3.5 text-left text-gray-400 font-semibold">Time</th>
                        </tr>
                    </thead>
                    <tbody className="divide-y divide-gray-500">
                        <tr key={"1"}>
                            <td className="whitespace-nowrap py-4 pl-4 pr-3 font-medium sm:pl-6">647B1266BC9D607EFFD5C0E93C81D7619D3F443D5E9BE703A50C7A76620B1739</td>
                            <td className="whitespace-nowrap px-3 py-4">MsgSend</td>
                            <td className="whitespace-nowrap px-3 py-4">Success</td>
                            <td className="whitespace-nowrap px-3 py-4">100</td>
                            <td className="whitespace-nowrap px-3 py-4">10 seconds ago</td>
                        </tr>
                    </tbody>
                </table>
            </div>
        </div>
    )
}

export default TransactionSubList;