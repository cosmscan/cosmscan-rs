import { Fragment } from 'react'
import { Disclosure, Menu, Transition } from '@headlessui/react'
import { MagnifyingGlassIcon } from '@heroicons/react/20/solid'
import { Bars3Icon, BellIcon, XMarkIcon } from '@heroicons/react/24/outline'
import UISelectChain from './selectChain'

function classNames(...classes: string[]) {
    return classes.filter(Boolean).join(' ')
}

const menus = [{
    name: "Dashboard",
    href: "/"
},{
    name: "Blocks",
    href: "/blocks",
}, {
    name: "Transactions",
    href: "/transactions",
}]

const Header = () => {
    return (
        <Disclosure as="nav">
            {({ open }) => (
                <>
                    <div className="mx-auto w-full lg:w-10/12 px-2 sm:px-4 lg:px-8">
                        <div className="flex h-16 justify-between">
                            <div className="flex px-2 lg:px-0">
                                <div className="flex flex-shrink-0 items-center">
                                    <a href="/" className="w-auto text-indigo-400 font-bold">Cosmscan</a>
                                </div>
                                <div className="hidden lg:ml-6 lg:flex lg:space-x-8">
                                    {/* Current: "border-indigo-500 text-gray-900", Default: "border-transparent text-gray-500 hover:border-gray-300 hover:text-gray-700" */}
                                    {menus.map((item) => (
                                        <a key={item.name} href={item.href} className="inline-flex items-center border-b-2 border-transparent px-1 pt-1 font-medium text-gray-300 hover:border-blue-200 hover:text-white">
                                            {item.name}
                                        </a>
                                    ))}
                                </div>
                            </div>
                            <div className="flex flex-1 items-center justfiy-center lg:justify-end">
                                <UISelectChain />
                            </div>
                            <div className="flex w-80 items-center justify-center px-2 lg:justify-end">
                                <div className="w-full max-w-lg lg:max-w-xs">
                                    <label htmlFor="search" className="sr-only">
                                        Search
                                    </label>
                                    <div className="relative">
                                        <div className="pointer-events-none absolute inset-y-0 left-0 flex items-center pl-3">
                                            <MagnifyingGlassIcon className="h-5 w-5 text-gray-400" aria-hidden="true" />
                                        </div>
                                        <input
                                            id="search"
                                            name="search"
                                            className="block w-full rounded-md border border-transparent bg-gray-700 py-2 pl-10 pr-3 leading-5 placeholder-gray-300 focus:border-indigo-500 focus:placeholder-gray-200 focus:outline-none focus:ring-1 focus:ring-indigo-500 text-white"
                                            placeholder="Search Block / Transaction / Account"
                                            type="search"
                                        />
                                    </div>
                                </div>
                            </div>
                            <div className="flex items-center lg:hidden">
                                {/* Mobile menu button */}
                                <Disclosure.Button className="inline-flex items-center justify-center rounded-md p-2 text-gray-400 hover:bg-gray-100 hover:text-gray-500 focus:outline-none focus:ring-2 focus:ring-inset focus:ring-indigo-500">
                                    <span className="sr-only">Open main menu</span>
                                    {open ? (
                                        <XMarkIcon className="block h-6 w-6" aria-hidden="true" />
                                    ) : (
                                        <Bars3Icon className="block h-6 w-6" aria-hidden="true" />
                                    )}
                                </Disclosure.Button>
                            </div>
                        </div>
                    </div>

                    <Disclosure.Panel className="lg:hidden">
                        <div className="space-y-1 pt-2 pb-3">
                            {/* Current: "bg-indigo-50 border-indigo-500 text-indigo-700", Default: "border-transparent text-gray-600 hover:bg-gray-50 hover:border-gray-300 hover:text-gray-800" */}
                            {menus.map((item) => (
                                <Disclosure.Button
                                    as="a"
                                    href={item.href}
                                    className="block border-l-4 border-transparent py-2 pl-3 pr-4 text-base font-medium hover:border-gray-300 hover:bg-gray-50 hover:text-gray-800"
                                >
                                    {item.name}
                                </Disclosure.Button>
                            ))}
                        </div>
                    </Disclosure.Panel>
                </>
            )}
        </Disclosure>
    )
}

export default Header;