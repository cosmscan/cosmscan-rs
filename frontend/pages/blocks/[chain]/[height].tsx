import type { NextPage } from "next";
import { PaperClipIcon } from '@heroicons/react/20/solid'
import Layout from "../../../components/layout";
import TransactionSubList from "../../../components/views/transactionSubList";

const ViewBlockPage:NextPage = () => {
    return (
        <Layout>
            <div className="mx-auto w-10/12 px-2 mt-16">
                <div className="sm:flex sm:items-center">
                    <h1 className="text-4xl font-semibold">Block Details</h1>
                </div>
                <div className="overflow-hidden bg-gray-800 shadow sm:rounded-lg mt-5">
                    <div className="px-4 py-5 sm:px-6">
                        <h3 className="text-lg font-medium leading-6">Overview</h3>
                    </div>
                    <div className="px-4 py-5 sm:p-0">
                        <dl>
                            <div className="py-4 sm:grid sm:grid-cols-4 sm:gap-4 sm:py-5 sm:px-6">
                                <dt className="font-medium text-gray-300">Height</dt>
                                <dd className="mt-1 sm:col-span-3 sm:mt-0">500</dd>
                            </div>
                            <div className="py-4 sm:grid sm:grid-cols-4 sm:gap-4 sm:py-5 sm:px-6">
                                <dt className="font-medium text-gray-300">Block Time</dt>
                                <dd className="mt-1 sm:col-span-3 sm:mt-0">April, 4 2022 10:24:22</dd>
                            </div>
                            <div className="py-4 sm:grid sm:grid-cols-4 sm:gap-4 sm:py-5 sm:px-6">
                                <dt className="font-medium text-gray-300">Block Hash</dt>
                                <dd className="mt-1 sm:col-span-3 sm:mt-0">1C919EED21DE4C690592962D6AFC66D63985317F3D254D95EAEEBE94D2F29D46</dd>
                            </div>
                            <div className="py-4 sm:grid sm:grid-cols-4 sm:gap-4 sm:py-5 sm:px-6">
                                <dt className="font-medium text-gray-300">Previous Block Hash</dt>
                                <dd className="mt-1 sm:col-span-3 sm:mt-0">1C919EED21DE4C690592962D6AFC66D63985317F3D254D95EAEEBE94D2F29D46</dd>
                            </div>
                            <div className="py-4 sm:grid sm:grid-cols-4 sm:gap-4 sm:py-5 sm:px-6">
                                <dt className="font-medium text-gray-300">Proposer</dt>
                                <dd className="mt-1 sm:col-span-3 sm:mt-0">cosmosvaloper1n229vhepft6wnkt5tjpwmxdmcnfz55jv3vp77d</dd>
                            </div>
                            <div className="py-4 sm:grid sm:grid-cols-4 sm:gap-4 sm:py-5 sm:px-6">
                                <dt className="font-medium text-gray-300">Transactions</dt>
                                <dd className="mt-1 sm:col-span-3 sm:mt-0">Total 1250</dd>
                            </div>
                        </dl>
                    </div>
                </div>
                <TransactionSubList />
            </div>
        </Layout>
    )
}

export default ViewBlockPage;