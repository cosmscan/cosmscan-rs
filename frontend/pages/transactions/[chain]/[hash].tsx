import type { NextPage } from "next";
import Layout from "../../../components/layout";

const ViewTransactionPage: NextPage = () => {
    return (
        <Layout>
            <div className="mx-auto w-10/12 px-2 mt-16">
                <div className="sm:flex sm:items-center">
                    <h1 className="text-4xl font-semibold">Transaction Details</h1>
                </div>
                <div className="overflow-hidden bg-gray-800 shadow sm:rounded-lg mt-5">
                    <div className="px-4 py-5 sm:px-6">
                        <h3 className="text-lg font-medium leading-6 text-green-300">Overview</h3>
                    </div>
                    <div className="px-4 py-5 sm:p-0">
                        <dl>
                            <div className="py-4 sm:grid sm:grid-cols-4 sm:gap-4 sm:py-5 sm:px-6">
                                <dt className="font-medium text-gray-500">TxHash</dt>
                                <dd className="mt-1 sm:col-span-3 sm:mt-0">1C919EED21DE4C690592962D6AFC66D63985317F3D254D95EAEEBE94D2F29D46</dd>
                            </div>
                            <div className="py-4 sm:grid sm:grid-cols-4 sm:gap-4 sm:py-5 sm:px-6">
                                <dt className="font-medium text-gray-500">Status</dt>
                                <dd className="mt-1 sm:col-span-3 sm:mt-0">Success</dd>
                            </div>
                            <div className="py-4 sm:grid sm:grid-cols-4 sm:gap-4 sm:py-5 sm:px-6">
                                <dt className="font-medium text-gray-500">Height</dt>
                                <dd className="mt-1 sm:col-span-3 sm:mt-0">100</dd>
                            </div>
                            <div className="py-4 sm:grid sm:grid-cols-4 sm:gap-4 sm:py-5 sm:px-6">
                                <dt className="font-medium text-gray-500">Time</dt>
                                <dd className="mt-1 sm:col-span-3 sm:mt-0">April, 4 2022 10:24:22</dd>
                            </div>
                            <div className="py-4 sm:grid sm:grid-cols-4 sm:gap-4 sm:py-5 sm:px-6">
                                <dt className="font-medium text-gray-500">Fee</dt>
                                <dd className="mt-1 sm:col-span-3 sm:mt-0">1.000000 ATOM</dd>
                            </div>
                            <div className="py-4 sm:grid sm:grid-cols-4 sm:gap-4 sm:py-5 sm:px-6">
                                <dt className="font-medium text-gray-500">Gas</dt>
                                <dd className="mt-1 sm:col-span-3 sm:mt-0">67000 used / 100000 wanted</dd>
                            </div>
                            <div className="py-4 sm:grid sm:grid-cols-4 sm:gap-4 sm:py-5 sm:px-6">
                                <dt className="font-medium text-gray-500">Memo</dt>
                                <dd className="mt-1 sm:col-span-3 sm:mt-0">-</dd>
                            </div>
                        </dl>
                    </div>
                    <div className="px-4 py-5 sm:px-1 mx-5 border-t border-gray-500">
                        <h3 className="text-lg font-medium leading-6">Messages</h3>
                    </div>
                    <div className="px-6 pb-5">
                        <div className="mt-5">
                            <h4 className="text-gray-400">#1 MsgSend</h4>
                            <div className="overflow-hidden bg-gray-900 shadow sm:rounded-lg mt-2 py-4">
                                <dl>
                                    <div className="py-2 sm:grid sm:grid-cols-5 sm:gap-4 sm:px-6">
                                        <dt className="font-medium text-gray-500">From Address</dt>
                                        <dd className="mt-1 sm:col-span-4 sm:mt-0">cosmos1qyqsyqcyq5rqwzqfpg9scrgjl03hn0y2ydfh5u</dd>
                                    </div>
                                    <div className="py-2 sm:grid sm:grid-cols-5 sm:gap-4 sm:px-6">
                                        <dt className="font-medium text-gray-500">To Address</dt>
                                        <dd className="mt-1 sm:col-span-4 sm:mt-0">cosmos1qyqsyqcyq5rqwzqfpg9scrgjl03hn0y2ydfh5u</dd>
                                    </div>
                                    <div className="py-2 sm:grid sm:grid-cols-5 sm:gap-4 sm:px-6">
                                        <dt className="font-medium text-gray-500">Amount</dt>
                                        <dd className="mt-1 sm:col-span-4 sm:mt-0">1.000 ATOM</dd>
                                    </div>
                                </dl>
                            </div>
                        </div>
                        <div className="mt-5">
                            <h4 className="text-gray-400">#1 MsgSend</h4>
                            <div className="overflow-hidden bg-gray-900 shadow sm:rounded-lg mt-2 py-4">
                                <dl>
                                    <div className="py-2 sm:grid sm:grid-cols-5 sm:gap-4 sm:px-6">
                                        <dt className="font-medium text-gray-500">From Address</dt>
                                        <dd className="mt-1 sm:col-span-4 sm:mt-0">cosmos1qyqsyqcyq5rqwzqfpg9scrgjl03hn0y2ydfh5u</dd>
                                    </div>
                                    <div className="py-2 sm:grid sm:grid-cols-5 sm:gap-4 sm:px-6">
                                        <dt className="font-medium text-gray-500">To Address</dt>
                                        <dd className="mt-1 sm:col-span-4 sm:mt-0">cosmos1qyqsyqcyq5rqwzqfpg9scrgjl03hn0y2ydfh5u</dd>
                                    </div>
                                    <div className="py-2 sm:grid sm:grid-cols-5 sm:gap-4 sm:px-6">
                                        <dt className="font-medium text-gray-500">Amount</dt>
                                        <dd className="mt-1 sm:col-span-4 sm:mt-0">1.000 ATOM</dd>
                                    </div>
                                </dl>
                            </div>
                        </div>
                        <div className="mt-5">
                            <h4 className="text-gray-400">#1 MsgSend</h4>
                            <div className="overflow-hidden bg-gray-900 shadow sm:rounded-lg mt-2 py-4">
                                <dl>
                                    <div className="py-2 sm:grid sm:grid-cols-5 sm:gap-4 sm:px-6">
                                        <dt className="font-medium text-gray-500">From Address</dt>
                                        <dd className="mt-1 sm:col-span-4 sm:mt-0">cosmos1qyqsyqcyq5rqwzqfpg9scrgjl03hn0y2ydfh5u</dd>
                                    </div>
                                    <div className="py-2 sm:grid sm:grid-cols-5 sm:gap-4 sm:px-6">
                                        <dt className="font-medium text-gray-500">To Address</dt>
                                        <dd className="mt-1 sm:col-span-4 sm:mt-0">cosmos1qyqsyqcyq5rqwzqfpg9scrgjl03hn0y2ydfh5u</dd>
                                    </div>
                                    <div className="py-2 sm:grid sm:grid-cols-5 sm:gap-4 sm:px-6">
                                        <dt className="font-medium text-gray-500">Amount</dt>
                                        <dd className="mt-1 sm:col-span-4 sm:mt-0">1.000 ATOM</dd>
                                    </div>
                                </dl>
                            </div>
                        </div>
                        <div className="mt-5">
                            <h4 className="text-gray-400">#1 MsgSend</h4>
                            <div className="overflow-hidden bg-gray-900 shadow sm:rounded-lg mt-2 py-4">
                                <dl>
                                    <div className="py-2 sm:grid sm:grid-cols-5 sm:gap-4 sm:px-6">
                                        <dt className="font-medium text-gray-500">From Address</dt>
                                        <dd className="mt-1 sm:col-span-4 sm:mt-0">cosmos1qyqsyqcyq5rqwzqfpg9scrgjl03hn0y2ydfh5u</dd>
                                    </div>
                                    <div className="py-2 sm:grid sm:grid-cols-5 sm:gap-4 sm:px-6">
                                        <dt className="font-medium text-gray-500">To Address</dt>
                                        <dd className="mt-1 sm:col-span-4 sm:mt-0">cosmos1qyqsyqcyq5rqwzqfpg9scrgjl03hn0y2ydfh5u</dd>
                                    </div>
                                    <div className="py-2 sm:grid sm:grid-cols-5 sm:gap-4 sm:px-6">
                                        <dt className="font-medium text-gray-500">Amount</dt>
                                        <dd className="mt-1 sm:col-span-4 sm:mt-0">1.000 ATOM</dd>
                                    </div>
                                </dl>
                            </div>
                        </div>
                        
                    </div>
                </div>
            </div>
        </Layout>
    )
}

export default ViewTransactionPage;