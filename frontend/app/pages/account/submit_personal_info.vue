<script setup lang="ts">
import type { FormSubmitEvent } from '@nuxt/ui'

const CTF_EXPERIENCE_OPTIONS = [
  { label: 'Brak doświadczenia', value: 'none' },
  { label: 'Początkujący', value: 'beginner' },
  { label: 'Średniozaawansowany', value: 'intermediate' },
  { label: 'Zaawansowany', value: 'advanced' },
]

const CTF_INTEREST_AREAS = [
  'Web',
  'Crypto',
  'Pwn',
  'Rev',
  'Forensics',
  'OSINT',
  'Misc',
]

const REFERRAL_SOURCES = [
  'Linkedin',
  'Facebook',
  'Instagram',
  'Znajomy',
  'Internet',
  'Inne',
]

enum Age {
  '10-13' = 10,
  '14-16' = 14,
  '16-18' = 16,
  '19-22' = 19,
  '23+' = 23,
}

const currentYear = new Date().getFullYear()
const ageKeys = Object.keys(Age).filter(key => Number.isNaN(Number(key)))

const step = ref(1)
const totalSteps = 3

const schema1 = z.object({
  first_name: z.string().nonempty({ error: 'Imię jest wymagane' }),
  age: z.enum(ageKeys, { message: 'Wiek jest wymagany' }),
  location: z.string().nonempty({ error: 'Lokalizacja jest wymagana' }),
  organization: z.string().nonempty({ error: 'Organizacja jest wymagana' }),
})

const schema2 = z.object({
  ctf_experience: z.enum(CTF_EXPERIENCE_OPTIONS.map(o => o.value), { message: 'Wybierz poziom doświadczenia' }),
  ctf_interest_areas: z.array(z.string()).optional(),
})

const schema3 = z.object({
  referral_source: z.array(z.enum(REFERRAL_SOURCES)).nonempty('Proszę wybrać co najmniej jedno źródło polecenia.'),
  marketing_consent: z.boolean().default(false),
})

const { data } = await useAuth('/account/get_personal_information', {
  onResponseError: undefined,
})

const isEditing = !!data.value

function getAge(birth_year: number): string | undefined {
  const age = currentYear - birth_year
  for (const [key, value] of Object.entries(Age).reverse()) {
    if (age >= Number(value)) {
      return key
    }
  }
  return undefined
}

const state1 = reactive({
  first_name: data.value?.first_name ?? '',
  age: data.value ? getAge(data.value.birth_year) : undefined,
  location: data.value?.location ?? '',
  organization: data.value?.organization ?? '',
})

const state2 = reactive({
  ctf_experience: (data.value as any)?.ctf_experience ?? undefined,
  ctf_interest_areas: (data.value as any)?.ctf_interest_areas ?? [],
})

const state3 = reactive({
  referral_source: (data.value?.referral_source as string[] | null) ?? [],
  marketing_consent: data.value?.marketing_consent ?? false,
})

function nextStep() {
  step.value = Math.min(step.value + 1, totalSteps)
}

function prevStep() {
  step.value = Math.max(step.value - 1, 1)
}

function onStep1Submit(_event: FormSubmitEvent<any>) {
  nextStep()
}

function onStep2Submit(_event: FormSubmitEvent<any>) {
  nextStep()
}

async function onFinalSubmit(_event: FormSubmitEvent<any>) {
  const birth_year = currentYear - Age[state1.age as keyof typeof Age]

  await useNuxtApp().$auth('/account/submit_personal_information', {
    method: 'POST',
    credentials: 'include',
    body: {
      first_name: state1.first_name,
      birth_year,
      location: state1.location,
      organization: state1.organization,
      is_vegetarian: false,
      marketing_consent: state3.marketing_consent,
      referral_source: state3.referral_source,
      ctf_experience: state2.ctf_experience,
      ctf_interest_areas: state2.ctf_interest_areas.length > 0 ? state2.ctf_interest_areas : undefined,
    },
  })

  useToast().add({ title: 'Sukces', description: isEditing ? 'Pomyślnie zaktualizowano dane' : 'Pomyślnie uzupełniono dane', color: 'success' })

  await refreshNuxtData()
  navigateTo('/panel')
}

function toggleInterestArea(area: string) {
  const idx = state2.ctf_interest_areas.indexOf(area)
  if (idx >= 0) {
    state2.ctf_interest_areas.splice(idx, 1)
  }
  else {
    state2.ctf_interest_areas.push(area)
  }
}

function toggleReferralSource(source: string) {
  const idx = state3.referral_source.indexOf(source)
  if (idx >= 0) {
    state3.referral_source.splice(idx, 1)
  }
  else {
    state3.referral_source.push(source)
  }
}
</script>

<template>
  <div class="md:w-170 sm:w-90 w-60">
    <h1 class="text-2xl font-medium mb-2">
      {{ isEditing ? 'Edytuj informacje o koncie' : 'Podaj dodatkowe informacje' }}
    </h1>

    <div class="text-sm mb-4 space-y-3">
      <p v-if="!isEditing">
        Zanim zaczniesz korzystać ze strony, podaj proszę dodatkowe informacje o koncie
      </p>
      <p>
        Dane, które tu udostępnisz, będą widoczne jedynie dla organizatorów wydarzenia i są konieczne do jego przeprowadzenia.
      </p>
    </div>

    <!-- Step indicator -->
    <div class="flex items-center gap-2 mb-6">
      <template v-for="s in totalSteps" :key="s">
        <div
          class="h-2 flex-1 rounded-full transition-colors"
          :class="s <= step ? 'bg-primary' : 'bg-neutral-200 dark:bg-neutral-700'"
        />
      </template>
    </div>
    <p class="text-sm text-neutral-500 mb-4">
      Krok {{ step }} z {{ totalSteps }}
    </p>

    <!-- Step 1: Personal info -->
    <UForm v-show="step === 1" :schema="schema1" :state="state1" class="space-y-4" @submit="onStep1Submit">
      <UFormField label="Imię" name="first_name">
        <UInput v-model="state1.first_name" class="w-full" placeholder="Twoje imię" />
      </UFormField>

      <UFormField label="Wiek" name="age">
        <USelect
          v-model="state1.age"
          class="w-full"
          :items="ageKeys.map(k => ({ label: k, value: k }))"
          placeholder="Wybierz przedział wiekowy"
        />
      </UFormField>

      <UFormField label="Lokalizacja" name="location" description="Miejscowość, w której mieszkasz">
        <UInput v-model="state1.location" class="w-full" placeholder="np. Kraków" />
      </UFormField>

      <UFormField label="Organizacja" name="organization" description="Twoja szkoła, uczelnia lub firma">
        <UInput v-model="state1.organization" class="w-full" placeholder="np. AGH" />
      </UFormField>

      <div class="flex justify-end pt-2">
        <UButton type="submit" label="Dalej" trailing-icon="i-lucide-arrow-right" />
      </div>
    </UForm>

    <!-- Step 2: CTF experience -->
    <UForm v-show="step === 2" :schema="schema2" :state="state2" class="space-y-4" @submit="onStep2Submit">
      <UFormField label="Doświadczenie z CTF" name="ctf_experience">
        <USelect
          v-model="state2.ctf_experience"
          class="w-full"
          :items="CTF_EXPERIENCE_OPTIONS"
          placeholder="Wybierz poziom doświadczenia"
        />
      </UFormField>

      <UFormField label="Obszary zainteresowań CTF" name="ctf_interest_areas" description="Wybierz kategorie, które Cię interesują (opcjonalne)">
        <div class="flex flex-wrap gap-2 mt-1">
          <UButton
            v-for="area in CTF_INTEREST_AREAS"
            :key="area"
            :label="area"
            :variant="state2.ctf_interest_areas.includes(area) ? 'solid' : 'outline'"
            :color="state2.ctf_interest_areas.includes(area) ? 'primary' : 'neutral'"
            size="sm"
            @click="toggleInterestArea(area)"
          />
        </div>
      </UFormField>

      <div class="flex justify-between pt-2">
        <UButton variant="ghost" label="Wstecz" leading-icon="i-lucide-arrow-left" @click="prevStep" />
        <UButton type="submit" label="Dalej" trailing-icon="i-lucide-arrow-right" />
      </div>
    </UForm>

    <!-- Step 3: Referral & marketing -->
    <UForm v-show="step === 3" :schema="schema3" :state="state3" class="space-y-4" @submit="onFinalSubmit">
      <UFormField label="Skąd się o nas dowiedziałeś? (wielokrotny wybór)" name="referral_source">
        <div class="flex flex-wrap gap-2 mt-1">
          <UButton
            v-for="source in REFERRAL_SOURCES"
            :key="source"
            :label="source"
            :variant="state3.referral_source.includes(source) ? 'solid' : 'outline'"
            :color="state3.referral_source.includes(source) ? 'primary' : 'neutral'"
            size="sm"
            @click="toggleReferralSource(source)"
          />
        </div>
      </UFormField>

      <div class="border rounded-lg p-4 bg-primary/5 border-primary/20">
        <UCheckbox
          v-model="state3.marketing_consent"
          label="Chcę dostawać informacje o kolejnych wydarzeniach organizowanych przez Hack4Krak"
        />
        <p class="text-xs text-neutral-500 mt-1 ml-7">
          Wyrażam zgodę na otrzymywanie informacji o nadchodzących wydarzeniach drogą mailową.
        </p>
      </div>

      <div class="flex justify-between pt-2">
        <UButton variant="ghost" label="Wstecz" leading-icon="i-lucide-arrow-left" @click="prevStep" />
        <UButton type="submit" :label="isEditing ? 'Zapisz zmiany' : 'Zapisz'" trailing-icon="i-lucide-check" />
      </div>
    </UForm>
  </div>
</template>
