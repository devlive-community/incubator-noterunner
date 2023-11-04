<template>
  <div>
    <TinyLayout :style="{backgroundColor: '#FFFFFF', height: height + 'px'}">
      <TinyTabs v-model="activeTab"
                tab-style="card"
                :with-close="true"
                @close="handlerClose"
                @click="handlerClick">
        <TinyTabItem v-for="item in tabs"
                     :key="item.key"
                     :title="item.title"
                     :style="{margin: 0, padding: 0}"
                     :name="item.name">
          <MarkdownEditor v-if="item.editor === 'Markdown'"
                          :style="{height: (height as number - 75) + 'px'}"
                          :content="item.content"
                          :key="item.key"
                          @onChange="handlerChange($event, item)">
          </MarkdownEditor>
        </TinyTabItem>
      </TinyTabs>
    </TinyLayout>
  </div>
</template>

<script lang="ts">
import {defineComponent} from "vue"
import {Layout as TinyLayout, TabItem as TinyTabItem, Tabs as TinyTabs} from '@opentiny/vue'
import MarkdownEditor from "../../components/MarkdownEditor.vue";
import {Note} from "../../model/note.ts";

export default defineComponent({
  name: 'LayoutContent',
  components: {MarkdownEditor, TinyLayout, TinyTabs, TinyTabItem},
  props: {
    height: {
      type: Number
    },
    note: {
      type: Object as () => Note
    }
  },
  data() {
    return {
      activeTab: '',
      tabs: Array<Note>()
    }
  },
  watch: {
    note() {
      const hasValue = this.tabs.filter(item => item.name === this.note?.name)
      if (hasValue.length === 0) {
        this.tabs.push(<Note>this.note)
      }
      const note = this.note as Note
      this.activeTab = note.key
    }
  },
  methods: {
    handlerChange(content: string, note: Note) {
      note.content = content
    },
    handlerClose(name: string) {
      const index = this.tabs.findIndex(item => item.name === name)
      if (this.tabs.length > 1) {
        const key = this.tabs[index === 0 ? index + 1 : index - 1].name
        this.activeTab = key
      } else {
        this.activeTab = this.tabs[0].name
      }
      this.tabs.splice(index, 1)
      this.handlerClick({name: this.activeTab})
    },
    handlerClick(tab: any) {
      const index = this.tabs.findIndex(item => item.name === tab.name)
      this.$emit('onClick', this.tabs[index])
    }
  }
});
</script>
<style scoped>
.tiny-tabs__content {
  padding: 0;
}
</style>
