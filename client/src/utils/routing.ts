import Home from '../pages/Home.svelte';
import Login from '../pages/Login.svelte';
import Signup from '../pages/Signup.svelte';

export interface IRoute {
    path: string;
    name: string;
    component: any;
    props?: object | undefined;
    isProtected?: boolean;
}

const routes: IRoute[] = [
    {
        path: '/',
        name: 'Home',
        component: Home,
        isProtected: true,
    },
    {
        path: '/login',
        name: 'Login',
        component: Login,
    },
    {
        path: '/signup',
        name: 'Sign up',
        component: Signup,
    },
];

export default routes;
