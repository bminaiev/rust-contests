use std::cmp::max;

use algo_lib::{collections::index_of::IndexOf, misc::rand::Random};

//START MAIN
use image::ImageBuffer;
//END MAIN

use crate::types::{CreatedVm, PlacementGroup, TestParams};

#[derive(Clone, Default)]
struct MachineState {
    alive_vm_ids: Vec<usize>,
}

impl MachineState {
    pub fn remove_vm(&mut self, vm_id: usize) {
        self.alive_vm_ids
            .remove(self.alive_vm_ids.index_of(&vm_id).unwrap());
    }
}

#[derive(Clone)]
pub struct State {
    params: TestParams,
    placement_groups: Vec<PlacementGroup>,
    vms: Vec<CreatedVm>,
    machines: Vec<MachineState>,
}

impl State {
    pub fn new(params: TestParams) -> Self {
        Self {
            machines: vec![MachineState::default(); params.total_machines()],
            params,
            placement_groups: vec![],
            vms: vec![],
        }
    }

    pub(crate) fn new_placement_group(&mut self, placement_group: crate::types::PlacementGroup) {
        self.placement_groups.push(placement_group);
    }

    pub(crate) fn register_new_vms(&mut self, res: &[CreatedVm]) {
        let first = self.vms.len();
        self.vms.extend(res.to_vec());
        for pos in first..self.vms.len() {
            self.machines[self.params.get_machine_id(&self.vms[pos].machine)]
                .alive_vm_ids
                .push(pos);
        }
    }

    pub(crate) fn delete_vms(&mut self, ids: &[usize]) {
        for &vm_id in ids.iter() {
            let machine_id = self.params.get_machine_id(&self.vms[vm_id].machine);
            self.machines[machine_id].remove_vm(vm_id);
        }
    }

    pub fn save_png(&self, path: &str) {
        //START MAIN
        const DC_HEIGHT_OFFSET: usize = 10;
        const MACHINE_HEIGHT_OFFSET: usize = 1;
        let one_machine_height = self.params.numa.len() * 2 + MACHINE_HEIGHT_OFFSET * 2;
        let one_rack_height = self.params.num_machines_per_rack * one_machine_height;

        let one_dc_height = DC_HEIGHT_OFFSET * 2 + (one_rack_height);

        const RACK_WIDTH_OFFSET: usize = 5;
        let one_rack_width = RACK_WIDTH_OFFSET * 2
            + (max(self.params.numa[0].cpu, self.params.numa[0].memory) as usize);

        let width = self.params.num_racks * one_rack_width;
        let height = one_dc_height * self.params.num_dc;
        let mut image = ImageBuffer::new(width as u32, height as u32);

        let mut set_pixel = |x: usize, y: usize, color: [u8; 3]| {
            *image.get_pixel_mut(x as u32, y as u32) = image::Rgb(color);
        };

        for x in 0..width {
            for y in 0..height {
                set_pixel(x, y, [255, 255, 255]);
            }
        }

        const GREY: [u8; 3] = [200, 200, 200];
        let mut rnd = Random::new(789993);
        let mut gen_color = || -> [u8; 3] { [rnd.gen(0..255), rnd.gen(0..255), rnd.gen(0..255)] };

        for dc in 0..self.params.num_dc {
            for rack in 0..self.params.num_racks {
                for inside_rack in 0..self.params.num_machines_per_rack {
                    let offset_x = rack * one_rack_width + RACK_WIDTH_OFFSET;
                    let offset_y = dc * one_dc_height
                        + DC_HEIGHT_OFFSET
                        + one_machine_height * inside_rack
                        + MACHINE_HEIGHT_OFFSET;
                    for numa_id in 0..self.params.numa.len() {
                        for dx in 0..self.params.numa[numa_id].cpu as usize {
                            set_pixel(offset_x + dx, offset_y + numa_id * 2, GREY);
                        }
                        for dx in 0..self.params.numa[numa_id].memory as usize {
                            set_pixel(offset_x + dx, offset_y + numa_id * 2 + 1, GREY);
                        }
                    }
                    let mut used = vec![0; self.params.numa.len() * 2];
                    let mut add = |id: usize, len: usize, color: [u8; 3]| {
                        for dx in 0..len {
                            set_pixel(offset_x + used[id] + dx, offset_y + id, color);
                        }
                        used[id] += len;
                    };
                    let m_id = self.params.get_machine_id2(dc, rack, inside_rack);
                    for &vm_id in self.machines[m_id].alive_vm_ids.iter() {
                        let color = gen_color();
                        let vm = &self.vms[vm_id];
                        for &numa_id in vm.numa_ids.iter() {
                            add(numa_id * 2, vm.spec.cpu as usize, color);
                            add(numa_id * 2 + 1, vm.spec.memory as usize, color);
                        }
                    }
                }
            }
        }

        // write it out to a file
        image.save(path).unwrap();

        //END MAIN
    }
}
