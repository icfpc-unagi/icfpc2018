use super::super::*;

pub fn destroy_small(model: Model) -> Vec<Command> {
    let r = model.r as i32;
    let mut all = vec![];

    //
    // Fission
    //
    let bot_ps = (0..8).map(|i| {
        P::new(
            ((i >> 0) & 1) * (r - 1),
            ((i >> 1) & 1) * (r - 1),
            ((i >> 2) & 1) * (r - 1))
    }).collect();
    let (order, commands) = fission_to(&model.filled, &bot_ps);
    all.extend(commands);

    //
    // GVoid
    //
    {
        let gvoid_ps: Vec<_> = (0..8).map(|i| {
            P::new(
                ((i >> 0) & 1) * (r - 1),
                ((i >> 1) & 1) * (r - 1),
                1 + ((i >> 2) & 1) * (r - 3))
        }).collect();

        let mut commands = vec![Command::Wait; 8];

        for i in 0..8 {
            let my_bid = order[i] - 1;  // ord is 1-indexed
            let my_bot_p = bot_ps[i];
            let my_gvoid_p = gvoid_ps[i];

            let opposite_gvoid_p = gvoid_ps[i ^ 7];
            commands[my_bid] = Command::GVoid(
                my_gvoid_p - my_bot_p,
                opposite_gvoid_p - my_gvoid_p,
            )
        }

        all.extend(commands);
    }

    //
    // Fusion
    //
    let mut bot_ps2 = vec![P::new(0, 0, 0); 8];
    for i in 0..8 {
        bot_ps2[order[i] - 1] = bot_ps[i];
    }
    let commands = postproc::fusion_all(&mat![false; r as usize; r as usize; r as usize], bot_ps2);
    all.extend(commands);

    return all;
}