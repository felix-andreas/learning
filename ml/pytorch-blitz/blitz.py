import torch
import torch.nn as nn
import torch.nn.functional as F


class Net(nn.Module):
    def __init__(self):
        super().__init__()
        self.conv_1 = nn.Conv2d(1, 6, 3)
        self.conv_2 = nn.Conv2d(6, 16, 3)
        self.fc_1 = nn.Linear(16 * 6 * 6, 120)
        self.fc_2 = nn.Linear(120, 84)
        self.fc_3 = nn.Linear(84, 10)

    def forward(self, x):
        x = F.max_pool2d(F.relu(self.conv_1(x)), (2, 2))
        x = F.max_pool2d(F.relu(self.conv_2(x)), 2)
        x = x.view(-1, self.num_flat_features(x))
        x = F.relu(self.fc_1(x))
        x = F.relu(self.fc_2(x))
        return self.fc_3(x)

    def num_flat_features(self, x):
        size = x.size()[1:]
        num_featuers = 1
        for s in size:
            num_featuers *= s
        return num_featuers


net = Net()
print(net)