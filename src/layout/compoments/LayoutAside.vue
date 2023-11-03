<template>
  <div>
    <TinyLayout
        :style="{height: height + 'px', padding: '10px 0px', textAlign: 'center', backgroundColor: '#FFFFFF'}">
      <TinyDropdown split-button
                    type="success">
        Add new file
        <template #dropdown>
          <TinyDropdownMenu>
            <TinyDropdownItem label="Markdown">Markdown</TinyDropdownItem>
          </TinyDropdownMenu>
        </template>
      </TinyDropdown>
      <TinyDivider :style="{margin: '12px 0px 0px 0px'}">
      </TinyDivider>
      <TinyTree :data="data"
                :show-line="true"
                :style="{height: height + 'px', width: width + 'px'}">
      </TinyTree>
    </TinyLayout>
  </div>
</template>

<script>
import {defineComponent} from "vue"
import {
  Divider as TinyDivider,
  Dropdown as TinyDropdown,
  DropdownItem as TinyDropdownItem,
  DropdownMenu as TinyDropdownMenu,
  Layout as TinyLayout,
  Notify,
  Tree as TinyTree
} from '@opentiny/vue'
import {invoke} from '@tauri-apps/api/tauri'

export default defineComponent({
  name: 'LayoutAside',
  components: {TinyLayout, TinyDropdown, TinyDropdownMenu, TinyDropdownItem, TinyDivider, TinyTree},
  props: {
    height: {
      type: Number
    },
    width: {
      type: Number
    }
  },
  data() {
    return {
      data: []
    }
  },
  created() {
    this.handlerInitialize()
  },
  methods: {
    handlerInitialize() {
      invoke('get_notes')
          .then(response => {
            if (response.code === 200) {
              this.data = response.data
            } else {
              Notify({
                type: 'error',
                message: response.message,
                position: 'top',
                title: '错误',
                duration: 1000
              })
            }
          })
    }
  }
});
</script>
