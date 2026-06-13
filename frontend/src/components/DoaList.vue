<script setup>
import { ref, onMounted, computed } from 'vue'
import axios from 'axios'

const allDoa = ref([])
const loading = ref(true)
const error = ref(null)
const searchQuery = ref('')
const activeTag = ref(null)
const expandedId = ref(null)

async function fetchDoa() {
  loading.value = true
  error.value = null
  try {
    const { data } = await axios.get('/api/doa')
    if (data.status === 'success') {
      allDoa.value = data.data
    }
  } catch (err) {
    error.value = 'Gagal memuat data. Pastikan backend berjalan di port 3001.'
  } finally {
    loading.value = false
  }
}

const allTags = computed(() => {
  const tags = new Set()
  allDoa.value.forEach(d => (d.tag || []).forEach(t => tags.add(t)))
  return [...tags].sort()
})

const filteredDoa = computed(() => {
  let list = allDoa.value
  if (activeTag.value) {
    list = list.filter(d => (d.tag || []).includes(activeTag.value))
  }
  const q = searchQuery.value.toLowerCase().trim()
  if (q) {
    list = list.filter(d =>
      d.nama.toLowerCase().includes(q) ||
      d.idn.toLowerCase().includes(q) ||
      (d.grup || '').toLowerCase().includes(q) ||
      (d.tag || []).some(t => t.includes(q))
    )
  }
  return list
})

function toggleCard(id) {
  expandedId.value = expandedId.value === id ? null : id
}

function setTag(tag) {
  activeTag.value = activeTag.value === tag ? null : tag
}

onMounted(fetchDoa)
</script>

<template>
  <div class="min-h-screen">
    <div class="max-w-3xl mx-auto px-4 sm:px-6">

      <!-- Header -->
      <header class="pt-16 pb-8 text-center">
        <p class="font-[Amiri] text-2xl text-emerald-400 mb-4">
          بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيمِ
        </p>
        <h1 class="text-3xl sm:text-4xl font-bold tracking-tight mb-2">
          Kumpulan <span class="text-emerald-400">Doa</span> Harian
        </h1>
        <p class="text-sm text-gray-500">
          228 doa dan dzikir dari API EQuran.id
        </p>
      </header>

      <!-- Search + Tags -->
      <div class="sticky top-0 z-10 bg-[#0a0f0d]/85 backdrop-blur-xl py-4 border-b border-white/5 mb-6">
        <div class="relative mb-3">
          <svg class="absolute left-4 top-1/2 -translate-y-1/2 w-4 h-4 text-gray-500"
               xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24"
               stroke="currentColor" stroke-width="2">
            <circle cx="11" cy="11" r="8"/><path d="m21 21-4.3-4.3"/>
          </svg>
          <input
            v-model="searchQuery"
            type="text"
            placeholder="Cari doa berdasarkan nama, terjemahan, atau tag..."
            class="w-full pl-11 pr-4 py-3 bg-[#111a16] border border-white/5
                   rounded-xl text-sm text-white placeholder:text-gray-600
                   outline-none focus:border-emerald-500/30 focus:ring-2
                   focus:ring-emerald-500/10 transition-all"
          />
        </div>
        <div class="flex gap-2 overflow-x-auto pb-1 no-scrollbar">
          <button
            @click="activeTag = null"
            :class="[
              'shrink-0 px-3.5 py-1.5 rounded-full text-xs font-mono border transition-all',
              activeTag === null
                ? 'bg-emerald-700 border-emerald-500 text-emerald-100'
                : 'bg-[#111a16] border-white/5 text-gray-500 hover:border-white/15'
            ]"
          >semua</button>
          <button
            v-for="tag in allTags"
            :key="tag"
            @click="setTag(tag)"
            :class="[
              'shrink-0 px-3.5 py-1.5 rounded-full text-xs font-mono border transition-all',
              activeTag === tag
                ? 'bg-emerald-700 border-emerald-500 text-emerald-100'
                : 'bg-[#111a16] border-white/5 text-gray-500 hover:border-white/15'
            ]"
          >{{ tag }}</button>
        </div>
      </div>

      <!-- Stats -->
      <div class="flex justify-between items-center text-xs font-mono text-gray-500 mb-4">
        <span>Menampilkan <span class="text-emerald-400">{{ filteredDoa.length }}</span> doa</span>
        <span v-if="activeTag">filter: {{ activeTag }}</span>
      </div>

      <!-- Loading -->
      <div v-if="loading" class="text-center py-20">
        <div class="w-8 h-8 border-2 border-white/10 border-t-emerald-400 rounded-full animate-spin mx-auto mb-4"></div>
        <p class="text-gray-500 text-sm">Memuat doa dari API...</p>
      </div>

      <!-- Error -->
      <div v-else-if="error" class="text-center py-20 text-red-400">
        <p>{{ error }}</p>
      </div>

      <!-- Empty -->
      <div v-else-if="filteredDoa.length === 0" class="text-center py-20 text-gray-500">
        <p class="text-3xl mb-3 opacity-40">?</p>
        <p>Tidak ditemukan doa yang cocok.</p>
      </div>

      <!-- Cards -->
      <div v-else class="flex flex-col gap-3 pb-20">
        <div
          v-for="doa in filteredDoa"
          :key="doa.id"
          @click="toggleCard(doa.id)"
          :class="[
            'bg-[#111a16] border rounded-xl overflow-hidden cursor-pointer',
            'transition-all duration-300 hover:border-emerald-500/20 hover:-translate-y-0.5',
            expandedId === doa.id ? 'border-emerald-600/40' : 'border-white/5'
          ]"
        >
          <!-- Card Header -->
          <div class="flex items-center gap-4 p-4 sm:p-5">
            <div class="shrink-0 w-10 h-10 flex items-center justify-center
                        bg-emerald-500/10 border border-emerald-600/30
                        rounded-lg font-mono text-xs text-emerald-400">
              {{ doa.id }}
            </div>
            <div class="flex-1 min-w-0">
              <div class="font-semibold text-[15px]">{{ doa.nama }}</div>
              <div class="text-xs font-mono text-gray-500 truncate">{{ doa.grup }}</div>
            </div>
            <div class="hidden sm:block font-[Amiri] text-lg text-emerald-400 truncate max-w-48"
                 dir="rtl">{{ doa.ar }}</div>
            <svg
              :class="['w-5 h-5 shrink-0 text-white/20 transition-transform duration-300',
                        expandedId === doa.id ? 'rotate-180' : '']"
              xmlns="http://www.w3.org/2000/svg" fill="none"
              viewBox="0 0 24 24" stroke="currentColor" stroke-width="2"
            ><path d="m6 9 6 6 6-6"/></svg>
          </div>

          <!-- Expandable Body -->
          <div
            :class="['overflow-hidden transition-all duration-300',
                     expandedId === doa.id ? 'max-h-[800px]' : 'max-h-0']"
          >
            <div class="px-4 sm:px-5 pb-5 space-y-4" @click.stop>
              <div class="font-[Amiri] text-2xl sm:text-3xl leading-relaxed text-right py-4 border-b border-white/5"
                   dir="rtl">{{ doa.ar }}</div>

              <div>
                <div class="text-[10px] font-mono uppercase tracking-widest text-emerald-700 mb-1">Transliterasi</div>
                <p class="text-sm text-gray-400 italic">{{ doa.tr }}</p>
              </div>

              <div>
                <div class="text-[10px] font-mono uppercase tracking-widest text-emerald-700 mb-1">Terjemahan</div>
                <p class="text-sm leading-relaxed">{{ doa.idn }}</p>
              </div>

              <div v-if="doa.tentang"
                   class="text-sm text-gray-400 leading-relaxed p-3 bg-emerald-500/5 rounded-lg border-l-3 border-emerald-700"
                   v-html="doa.tentang.replace(/\n/g, '<br>')"></div>

              <div v-if="doa.tag?.length" class="flex gap-1.5 flex-wrap">
                <span v-for="t in doa.tag" :key="t"
                      class="px-2.5 py-0.5 bg-emerald-500/10 rounded-full text-[11px] font-mono text-emerald-400">
                  {{ t }}
                </span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.no-scrollbar::-webkit-scrollbar { display: none; }
.no-scrollbar { scrollbar-width: none; }
.border-l-3 { border-left-width: 3px; }
</style>
