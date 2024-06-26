<script setup>
import Blog from './components/Blog.vue'
</script>

<template>
    <main class="container mx-auto">
        <img src="/logo2.svg" alt="DFINITY logo" />
        <br />
        <br />
        <Blog />
    </main>
</template>

<!-- import { ref } from "vue"
import { dzien2_backend } from "declarations/dzien2_backend/index"


let greeting = ref("")

async function handleSubmit(e) {
    e.preventDefault()
    const target = e.target
    const name = target.querySelector("#name").value
    const numer = target.querySelector("#numer").value
    await dzien2_backend.greet(name, Number(numer)).then((response) => {
        greeting.value = response
    })
}

<h1 class="text-blue-600"> Cześć czytelniku mojego bloga *_*</h1>
        <form action="#" @submit="handleSubmit">
            <label for="name">Enter your name: &nbsp;</label>
            <input id="name" alt="Name" type="text" />
            <input id="numer" alt="Numer" type="number" />
            <button type="submit">Click Me!</button>
        </form>
        <section id="greeting">{{ greeting }}</section> -->