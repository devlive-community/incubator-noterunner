<template>
  <div>
    <TinyLayout :style="{backgroundColor: '#FFFFFF', height: height + 'px'}">
      <TinyTabs v-model="activeName"
                tab-style="card"
                :with-close="true">
        <TinyTabItem :key="item.name"
                     v-for="item in tabs"
                     :title="item.title"
                     :name="item.name">
          {{ item.content }}
        </TinyTabItem>
      </TinyTabs>
    </TinyLayout>
  </div>
</template>

<script>
import {defineComponent} from "vue"
import {Layout as TinyLayout, TabItem as TinyTabItem, Tabs as TinyTabs} from '@opentiny/vue'

export default defineComponent({
  name: 'LayoutHeader',
  components: {TinyLayout, TinyTabs, TinyTabItem},
  props: {
    height: {
      type: Number
    }
  },
  data() {
    return {
      activeName: 'first',
      tabs: [
        {
          title: '文章一',
          name: 'first',
          content: '文章一的内容'
        },
        {
          title: '文章二',
          name: 'second',
          content: '文章二的内容'
        }
      ]
    }
  }
});
</script>
